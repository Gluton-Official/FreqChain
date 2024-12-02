use nih_plug::prelude::*;
use nih_plug::util::window;
use nih_plug::util::StftHelper;
use realfft::num_complex::Complex32;
use realfft::num_traits::clamp_min;
use realfft::num_traits::Zero;
use crate::modules::smoother::Smoother;
use crate::modules::smoother::SmootherParams;
use crate::util::fft;
use crate::util::fft::ForwardFFT;
use crate::util::fft::InverseFFT;

const SIDECHAIN_INPUTS: usize = 1;
/// Number of samples to collect before processing in the STFT
const FFT_WINDOW_SIZE: usize = 1024;
/// Number of samples the STFT processes at a time within the window
const FFT_HOP_SIZE: usize = 128; // TODO: make debug parameter
const FFT_OVERLAP_TIMES: usize = FFT_WINDOW_SIZE / FFT_HOP_SIZE;
const GAIN_COMPENSATION: f32 = 1.0 / (FFT_WINDOW_SIZE * FFT_OVERLAP_TIMES) as f32;

pub struct FrequencySidechain {
    stft: StftHelper<SIDECHAIN_INPUTS>,

    channels: usize,
    window_size: usize,
    /// Number of sample blocks the STFT process separately within the window
    overlap_times: usize,
    gain_compensation: f32,

    forward_fft: ForwardFFT<f32>,
    inverse_fft: InverseFFT<f32>,

    main_complex_buffer: Vec<Complex32>,
    sidechain_complex_buffer: Vec<Vec<Complex32>>,

    window_function: Vec<f32>,

    smoother: Vec<Vec<Smoother>>
}

#[derive(Params)]
pub struct FrequencySidechainParams {
    #[id = "detail"]
    pub detail: FloatParam,
    #[id = "precision"]
    pub precision: FloatParam,

    #[nested(group = "Smoother")]
    pub smoother: SmootherParams,
}

impl FrequencySidechain {
    pub fn new(channels: usize, window_size: usize, hop_size: usize) -> Self {
        let (forward_fft, inverse_fft) = fft::create_fft_pair(window_size);

        let overlap_times = window_size / hop_size;

        Self {
            stft: StftHelper::new(channels, window_size, 0),

            channels,
            window_size,
            overlap_times,
            gain_compensation: ((window_size * overlap_times) as f32).recip(),

            forward_fft,
            inverse_fft,

            main_complex_buffer: fft::create_complex_buffer(window_size),
            // since the sidechain buffer's FFT is processed before the main buffer's FFT,
            // we need a buffer to store the results for each channel
            sidechain_complex_buffer: (0..channels)
                .map(|_| fft::create_complex_buffer(window_size))
                .collect(),
            
            window_function: window::hann(window_size),

            smoother: vec![vec![Smoother::default(); window_size]; channels],
        }
    }

    // TODO: investigate debug profiling for some type of flame graph / do some timed tests
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
                    // Apply the Hann windowing function
                    window::multiply_with_window(real_buffer, &self.window_function);
                    
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

                        self.smoother[channel_index][bin_index]
                            .process(&mut sidechain_frequency_magnitude, sample_rate, &params.smoother);
    
                        let result_magnitude = clamp_min(
                            frequency_magnitude - (sidechain_frequency_magnitude * self.overlap_times as f32),
                            0.0,
                        );

                        // Reconstruct the complex value from the main input's phase and our output magnitude
                        // and apply gain compensation based on the FFT size
                        *main_bin = Complex32::from_polar(result_magnitude, phase) * self.gain_compensation;
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
        self.stft.set_block_size(self.window_size);
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

#[cfg(test)]
mod tests {
    use crate::util::buffer_utils::BufferUtils;
    use super::*;

    const SAMPLE_RATE: f32 = 44100.0;

    fn create_buffer(buffers: Vec<Vec<f32>>) -> Buffer<'static> {
        assert_eq!(buffers.len(), 2);
        assert_eq!(buffers[0].len(), buffers[1].len());

        let num_samples = buffers[0].len();

        let buffer_slices = buffers
            .into_iter()
            .map(|inner| {
                let slice = inner.into_boxed_slice();
                let slice_ptr = Box::into_raw(slice);
                unsafe { &mut *slice_ptr }
            })
            .collect();

        let mut buffer = Buffer::default();
        unsafe {
            buffer.set_slices(num_samples, |output_slices| {
                *output_slices = buffer_slices;
            });
        }
        buffer
    }

    /// ### Arguments for F
    /// - `channel`
    /// - `sample_index`
    fn create_buffer_with(num_samples: usize, f: impl Fn(usize, usize) -> f32) -> Buffer<'static> {
        let buffers = (0..2).map(|channel| {
            let mut buffer = vec![0_f32; num_samples];
            for sample_index in 0..num_samples {
                buffer[sample_index] = f(channel, sample_index);
            }
            buffer
        }).collect();
        create_buffer(buffers)
    }

    fn create_empty_buffer(num_samples: usize) -> Buffer<'static> {
        create_buffer(vec![vec![0.0; num_samples]; 2])
    }

    #[test]
    fn test_empty_buffers() {
        let mut fs = FrequencySidechain::new(2, 1024, 128);
        let mut main_buffer = create_empty_buffer(1024);
        let mut sidechain_buffer = create_empty_buffer(1024);
        fs.process(&mut main_buffer, &mut sidechain_buffer, SAMPLE_RATE, &FrequencySidechainParams::default());
        main_buffer.on_each_sample(|channel, sample_index, sample| {
            assert_eq!(*sample, 0.0, "Channel {} | Sample {}", channel, sample_index);
        });
    }

    #[test]
    fn test_equal_buffers() {
        let mut fs = FrequencySidechain::new(2, 1024, 128);
        let frequency = 440_f32;
        let mut main_buffer = create_buffer_with(1024, |_, sample_index| {
            ((frequency / SAMPLE_RATE) * sample_index as f32).sin()
        });
        let mut sidechain_buffer = create_buffer_with(1024, |_, sample_index| {
            ((frequency / SAMPLE_RATE) * sample_index as f32).sin()
        });
        fs.process(&mut main_buffer, &mut sidechain_buffer, SAMPLE_RATE, &FrequencySidechainParams::default());
        main_buffer.on_each_sample(|channel, sample_index, sample| {
            assert_eq!(*sample, 0.0, "Channel {} | Sample {}", channel, sample_index);
        });
    }
    
    #[test]
    fn test_remove_1_of_2_frequencies() {
        let mut fs = FrequencySidechain::new(2, 1024, 128);
        let frequency_1 = 440_f32;
        let frequency_2 = 880_f32;
        let mut main_buffer = create_buffer_with(1024, |_, sample_index| {
            ((frequency_1 / SAMPLE_RATE) * sample_index as f32).sin() + ((frequency_2 / SAMPLE_RATE) * sample_index as f32).sin()
        });
        let mut sidechain_buffer = create_buffer_with(1024, |_, sample_index| {
            ((frequency_2 / SAMPLE_RATE) * sample_index as f32).sin()
        });
        fs.process(&mut main_buffer, &mut sidechain_buffer, SAMPLE_RATE, &FrequencySidechainParams::default());
        main_buffer.on_each_sample(|channel, sample_index, sample| {
            assert_eq!(*sample, 0.0, "Channel {} | Sample {}", channel, sample_index);
            // assert_eq!(*sample, ((frequency_1 / SAMPLE_RATE) * sample_index as f32).sin(), "Channel {} | Sample {}", channel, sample_index);
        });
    }
}