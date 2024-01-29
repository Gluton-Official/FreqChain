use std::sync::Arc;

use nih_plug::{
    prelude::*,
    util::{self, window::multiply_with_window},
};
use realfft::{num_complex::Complex32, RealFftPlanner, RealToComplex};
use triple_buffer::TripleBuffer;

use crate::audio_processing::{fft, DEFAULT_CHANNELS};

/// The amplitudes of all the frequency bins from a windowed FFT
pub type SpectrumBuffer = [f32; fft::BUFFER_SIZE];
pub type SpectrumOutput = triple_buffer::Output<SpectrumBuffer>;

pub struct Spectrum {
    /// Scratch buffer that is copied into `triple_buffer_input` when finished computing
    spectrum_result_buffer: SpectrumBuffer,
    /// Receives data from `spectrum_result_buffer` and passes it to its linked output buffer
    triple_buffer_input: triple_buffer::Input<SpectrumBuffer>,

    num_channels: usize,

    /// Manages windowing for the FFT
    stft: util::StftHelper,
    /// FFT settings
    fft_plan: Arc<dyn RealToComplex<f32>>,
    /// A Hann window window for the STFT helper
    compensated_window_function: Vec<f32>,
    /// FFT output buffer
    complex_fft_buffer: Vec<Complex32>,

    smoothing_decay_weight: f32,
}

impl Spectrum {
    pub fn new() -> (Self, SpectrumOutput) {
        let (triple_buffer_input, triple_buffer_output) = TripleBuffer::new(&[0.0; fft::BUFFER_SIZE]).split();
        let spectrum_result_buffer = [0.0; fft::BUFFER_SIZE];

        let stft = util::StftHelper::new(DEFAULT_CHANNELS, fft::LENGTH, 0);
        let fft_plan = RealFftPlanner::new().plan_fft_forward(fft::LENGTH);
        let compensated_window_function = util::window::hann(fft::LENGTH)
            .into_iter()
            .map(|x| x / fft::LENGTH as f32)
            .collect();
        let complex_fft_buffer = vec![Complex32::default(); fft::BUFFER_SIZE];

        let spectrum = Self {
            spectrum_result_buffer,
            triple_buffer_input,

            num_channels: stft.num_channels(),

            stft,
            fft_plan,
            compensated_window_function,
            complex_fft_buffer,

            smoothing_decay_weight: 0.0,
        };

        (spectrum, triple_buffer_output)
    }

    pub fn set_smoothing_decay_weight(&mut self, smoothing_decay_weight: f32) {
        self.smoothing_decay_weight = smoothing_decay_weight;
    }

    pub fn num_channels(&self) -> usize {
        self.num_channels
    }

    pub fn set_num_channels(&mut self, num_channels: usize) {
        self.num_channels = num_channels;
        self.stft = util::StftHelper::new(num_channels, fft::LENGTH, 0);
    }

    pub fn compute(&mut self, buffer: &Buffer) {
        self.stft
            .process_analyze_only(buffer, fft::WINDOW_OVERLAP, |_channel_idx, real_fft_scratch_buffer| {
                multiply_with_window(real_fft_scratch_buffer, &self.compensated_window_function);

                self.fft_plan
                    .process_with_scratch(real_fft_scratch_buffer, &mut self.complex_fft_buffer, &mut [])
                    .unwrap();
            });

        self.update_spectrum();
        self.triple_buffer_input.write(self.spectrum_result_buffer);
    }

    fn update_spectrum(&mut self) {
        // Simulate peak meter behavior for each frequency bin
        // Quasi-mono summing occurs since this runs on all channels
        for (bin, spectrum_result) in self.complex_fft_buffer.iter().zip(&mut self.spectrum_result_buffer) {
            let magnitude = bin.norm();
            if magnitude > *spectrum_result {
                *spectrum_result = magnitude;
            } else {
                *spectrum_result = (*spectrum_result * self.smoothing_decay_weight)
                    + (magnitude * (1.0 - self.smoothing_decay_weight));
            }
        }
    }
}
