use nih_plug::{
    buffer::Buffer,
    util::{
        window::{hann, multiply_with_window},
        StftHelper,
    },
};
use realfft::{
    num_complex::Complex32,
    num_traits::{clamp_min, Zero},
};

use crate::audio_processing::fft::{self, ForwardFFT, InverseFFT};

const SIDECHAIN_INPUTS: usize = 1;
const DEFAULT_CHANNELS: usize = 2;
const FFT_WINDOW_SIZE: usize = 1024;
const FFT_HOP_SIZE: usize = 64;

const FFT_OVERLAP_TIMES: usize = FFT_WINDOW_SIZE / FFT_HOP_SIZE;
const GAIN_COMPENSATION: f32 = 1.0 / (FFT_WINDOW_SIZE * FFT_OVERLAP_TIMES) as f32;

pub struct FrequencySidechain {
    stft: StftHelper<SIDECHAIN_INPUTS>,

    channels: usize,

    forward_fft: ForwardFFT<f32>,
    inverse_fft: InverseFFT<f32>,

    main_complex_buffer: Vec<Complex32>,
    sidechain_complex_buffer: Vec<Vec<Complex32>>,

    window_function: Vec<f32>,
}

impl FrequencySidechain {
    pub fn new() -> Self {
        let (forward_fft, inverse_fft) = fft::create_fft_pair(FFT_WINDOW_SIZE);

        let channels = DEFAULT_CHANNELS;

        Self {
            stft: StftHelper::new(channels, FFT_WINDOW_SIZE, 0),

            channels,

            forward_fft,
            inverse_fft,

            main_complex_buffer: fft::create_complex_buffer(FFT_WINDOW_SIZE),
            sidechain_complex_buffer: (0..channels)
                .map(|_| fft::create_complex_buffer(FFT_WINDOW_SIZE))
                .collect(),

            window_function: hann(FFT_WINDOW_SIZE),
        }
    }

    pub fn ensure_channels(&mut self, channels: usize) {
        if channels != self.channels {
            self.channels = channels;
            self.stft = StftHelper::new(channels, FFT_WINDOW_SIZE, 0);
            self.main_complex_buffer = fft::create_complex_buffer(FFT_WINDOW_SIZE);
            self.sidechain_complex_buffer = (0..channels)
                .map(|_| fft::create_complex_buffer(FFT_WINDOW_SIZE))
                .collect();
        }
    }

    pub fn latency_samples(&self) -> u32 {
        self.stft.latency_samples()
    }

    pub fn reset(&mut self) {
        self.stft.set_block_size(FFT_WINDOW_SIZE);
    }

    pub fn process(&mut self, main_buffer: &mut Buffer, sidechain_buffer: &mut Buffer) {
        // Accumulates samples until the block size (our FFT size) is reached, then runs the callback
        self.stft.process_overlap_add_sidechain(
            main_buffer,
            [sidechain_buffer],
            FFT_OVERLAP_TIMES,
            |channel_index, sidechain_buffer_index, real_buffer| {
                // The sidechain buffers are be processed before the main buffer.
                // Since we only need a single sidechain buffer, we don't have to worry about the
                // sidechain buffer index's value
                if sidechain_buffer_index.is_some() {
                    // Apply the Hann windowing function
                    multiply_with_window(real_buffer, &self.window_function);

                    self.forward_fft
                        .process(real_buffer, &mut self.sidechain_complex_buffer[channel_index]);
                } else {
                    self.forward_fft.process(real_buffer, &mut self.main_complex_buffer);

                    for (main_bin, sidechain_bin) in self
                        .main_complex_buffer
                        .iter_mut()
                        // Use the relevant channel of our already processed sidechain buffer
                        .zip(&self.sidechain_complex_buffer[channel_index])
                    {
                        let (frequency_magnitude, phase) = main_bin.to_polar();
                        let sidechain_frequency_magnitude = sidechain_bin.norm();

                        let result_magnitude = clamp_min(
                            frequency_magnitude - (sidechain_frequency_magnitude * FFT_OVERLAP_TIMES as f32),
                            0.0,
                        );

                        // Reconstruct the complex value from the main input's phase and our output magnitude
                        // and apply gain compensation based on the FFT size
                        *main_bin = Complex32::from_polar(result_magnitude, phase) * GAIN_COMPENSATION;
                    }

                    // Set the imaginary values of the first and last frequency bins to zero
                    // so the inverse FFT can be performed
                    // TODO: prevent from having to set these?
                    self.main_complex_buffer.first_mut().unwrap().im.set_zero();
                    self.main_complex_buffer.last_mut().unwrap().im.set_zero();

                    self.inverse_fft.process(&mut self.main_complex_buffer, real_buffer);
                }
            },
        );
    }
}
