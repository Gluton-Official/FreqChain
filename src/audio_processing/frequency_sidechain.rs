use nih_plug::prelude::*;
use nih_plug::util::StftHelper;
use nih_plug::util::window;
use realfft::{
    num_complex::Complex32,
    num_traits::{clamp_min, Zero},
};

use crate::audio_processing::fft::{self, ForwardFFT, InverseFFT};
use crate::freqchain::CHANNELS;
use crate::audio_processing::smoother::{Smoother, SmootherParams};

const SIDECHAIN_INPUTS: usize = 1;
const DEFAULT_CHANNELS: usize = 2;
const FFT_WINDOW_SIZE: usize = 1024;
// Number of samples the STFT processes
const FFT_HOP_SIZE: usize = 64; // TODO: make debug parameter

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

    smoother: Vec<Smoother>
}

#[derive(Params)]
pub struct FrequencySidechainParams {
    #[id = "detail"]
    detail: FloatParam,
    #[id = "precision"]
    precision: FloatParam,

    #[nested(group = "Smoother")]
    smoother: SmootherParams,
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

            window_function: window::hann(FFT_WINDOW_SIZE),

            smoother: vec![Smoother::default(); FFT_WINDOW_SIZE],
        }
    }

    pub fn process(&mut self, main_buffer: &mut Buffer, sidechain_buffer: &mut Buffer, sample_rate: f32, params: &FrequencySidechainParams) {
        // Accumulates samples until the block size (our FFT size) is reached, then runs the callback
        self.stft.process_overlap_add_sidechain(
            main_buffer,
            [sidechain_buffer],
            FFT_OVERLAP_TIMES,
            // Processes each sidechain buffers' channels, then the main buffer's channels
            |channel_index, sidechain_buffer_index, real_buffer| {
                // The sidechain buffers are be processed before the main buffer.
                // Since we only have a single sidechain buffer, we only need to know if a sidechain_buffer_index
                // is provided
                if sidechain_buffer_index.is_some() {
                    // Apply the Hann windowing function
                    window::multiply_with_window(real_buffer, &self.window_function);

                    self.forward_fft.process(real_buffer, &mut self.sidechain_complex_buffer[channel_index]);
                } else {
                    // If no sidechain_buffer_index is provided, real_buffer is channel_index of main_buffer
                    self.forward_fft.process(real_buffer, &mut self.main_complex_buffer);

                    // Iterates frequency bins of the main buffer and sidechain buffer
                    for (bin_index, (main_bin, sidechain_bin)) in self
                        .main_complex_buffer
                        .iter_mut()
                        // Use the relevant channel of our already processed sidechain buffer
                        .zip(&self.sidechain_complex_buffer[channel_index])
                        .enumerate()
                    {
                        let (frequency_magnitude, phase) = main_bin.to_polar();
                        let mut sidechain_frequency_magnitude = sidechain_bin.norm();

                        self.smoother[bin_index].process(&mut sidechain_frequency_magnitude, sample_rate, &params.smoother);

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
                    self.main_complex_buffer.first_mut().unwrap().im.set_zero();
                    self.main_complex_buffer.last_mut().unwrap().im.set_zero();

                    self.inverse_fft.process(&mut self.main_complex_buffer, real_buffer);
                }
            },
        );
    }

    pub fn latency_samples(&self) -> u32 {
        self.stft.latency_samples()
    }

    pub fn reset(&mut self) {
        self.stft.set_block_size(FFT_WINDOW_SIZE);
    }
}

impl Default for FrequencySidechainParams {
    fn default() -> Self {
        Self {
            detail: FloatParam::new("Detail", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_unit("%")
                .with_value_to_string(formatters::v2s_f32_percentage(0))
                .with_string_to_value(formatters::s2v_f32_percentage()),
            precision: FloatParam::new("Precision", 0.8, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_unit("%")
                .with_value_to_string(formatters::v2s_f32_percentage(0))
                .with_string_to_value(formatters::s2v_f32_percentage()),

            smoother: SmootherParams::default(),
        }
    }
}