
use nih_plug::prelude::*;
use nih_plug::util::window;
use nih_plug::util::StftHelper;
use realfft::num_complex::{Complex, Complex32};
use realfft::num_traits::clamp_min;
use realfft::num_traits::Zero;
use crate::modules::smoother::Smoother;
use crate::modules::smoother::SmootherParams;
use crate::util::buffer_utils::BufferUtils;
use crate::util::fft;
use crate::util::fft::ForwardFFT;
use crate::util::fft::InverseFFT;

const SIDECHAIN_INPUTS: usize = 1;

pub struct FrequencySidechain<const CHANNELS: usize, const WINDOW_SIZE: usize, const HOP_SIZE: usize, const SPECTRUM_SIZE: usize> {
    stft: StftHelper<SIDECHAIN_INPUTS>,

    /// Number of sample blocks the STFT process separately within the window
    overlap_times: usize,
    cola: f32,

    forward_fft: ForwardFFT<f32>,
    inverse_fft: InverseFFT<f32>,

    main_complex_buffer: Vec<Complex32>,
    sidechain_complex_buffer: Vec<Vec<Complex32>>,

    window_function: Vec<f32>,

    smoother: Vec<Vec<Smoother>>,

    input_buffer_in: Option<triple_buffer::Input<[[Complex32; SPECTRUM_SIZE]; CHANNELS]>>,
    sidechain_buffer_in: Option<triple_buffer::Input<[[Complex32; SPECTRUM_SIZE]; CHANNELS]>>,
    output_buffer_in: Option<triple_buffer::Input<[[Complex32; SPECTRUM_SIZE]; CHANNELS]>>,
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

impl<const CHANNELS: usize, const WINDOW_SIZE: usize, const HOP_SIZE: usize, const SPECTRUM_SIZE: usize> FrequencySidechain<CHANNELS, WINDOW_SIZE, HOP_SIZE, SPECTRUM_SIZE> {
    pub fn new() -> Self {
        let (forward_fft, inverse_fft) = fft::create_fft_pair(WINDOW_SIZE);

        let overlap_times = WINDOW_SIZE / HOP_SIZE;
        let window_function = window::hann(WINDOW_SIZE);

        nih_log!("hann window sum: {}", window_function.iter().sum::<f32>());
        nih_log!("gain compensation factor: {}", HOP_SIZE as f32 / window_function.iter().sum::<f32>());

        Self {
            stft: StftHelper::new(CHANNELS, WINDOW_SIZE, 0),

            overlap_times,
            cola: HOP_SIZE as f32 / window_function.iter().sum::<f32>(),

            forward_fft,
            inverse_fft,

            main_complex_buffer: fft::create_complex_buffer(WINDOW_SIZE),
            // since the sidechain buffer's FFT is processed before the main buffer's FFT,
            // we need a buffer to store the results for each channel
            sidechain_complex_buffer: (0..CHANNELS)
                .map(|_| fft::create_complex_buffer(WINDOW_SIZE))
                .collect(),
            
            window_function,

            smoother: vec![vec![Smoother::default(); WINDOW_SIZE]; CHANNELS],

            sidechain_buffer_in: None,
            input_buffer_in: None,
            output_buffer_in: None,
        }
    }

    // TODO: investigate debug profiling for some type of flame graph / do some timed tests
    pub fn process(&mut self, main_buffer: &mut Buffer, sidechain_buffer: &mut Buffer, sample_rate: f32, params: &FrequencySidechainParams) {
        // Accumulates samples until the block size (our FFT size) is reached, then runs the callback
        self.stft.process_overlap_add_sidechain(
            main_buffer,
            [sidechain_buffer],
            self.overlap_times,
            // Processes each sidechain buffers' channels, then the main buffer's channels
            |channel_index, sidechain_buffer_index, real_buffer| {
                // The sidechain buffers are be processed before the main buffer.
                // Since we only have a single sidechain buffer, we only need to know if a sidechain_buffer_index
                // is provided
                if sidechain_buffer_index.is_some() {
                    // Apply the Hann windowing function
                    window::multiply_with_window(real_buffer, &self.window_function);

                    self.forward_fft.process(real_buffer, &mut self.sidechain_complex_buffer[channel_index]);

                    detail_smoothing(params.detail.value(), &mut self.sidechain_complex_buffer[channel_index], sample_rate);
                    precision_scaling(params.precision.value(), &mut self.sidechain_complex_buffer[channel_index]);

                    self.sidechain_complex_buffer.on_each_by_channel(channel_index, |bin_index, sidechain_sample| {
                        let (mut magnitude, phase) = sidechain_sample.to_polar();
                        self.smoother[channel_index][bin_index].process(&mut magnitude, sample_rate, &params.smoother);
                        *sidechain_sample = Complex::from_polar(magnitude, phase);
                    });

                    if let Some(sidechain_buffer_in) = self.sidechain_buffer_in.as_mut() {
                        let buffer = sidechain_buffer_in.input_buffer();
                        buffer[channel_index].copy_from_slice(self.sidechain_complex_buffer[channel_index].as_slice());

                        // publish if all channels have been supplied
                        if channel_index == CHANNELS - 1 {
                            sidechain_buffer_in.publish();
                        }
                    }
                } else {
                    // Apply the Hann windowing function
                    window::multiply_with_window(real_buffer, &self.window_function);
                    
                    // If no sidechain_buffer_index is provided, real_buffer is channel_index of main_buffer
                    self.forward_fft.process(real_buffer, &mut self.main_complex_buffer);

                    if let Some(input_buffer_in) = self.input_buffer_in.as_mut() {
                        let buffer = input_buffer_in.input_buffer();
                        buffer[channel_index].copy_from_slice(self.main_complex_buffer.as_slice());

                        // publish if both channels have been supplied
                        if channel_index == CHANNELS - 1 {
                            input_buffer_in.publish();
                        }
                    }

                    // Iterates frequency bins of the main buffer and sidechain buffer
                    for (main_bin, sidechain_bin) in self
                        .main_complex_buffer
                        .iter_mut()
                        // Use the relevant channel of our already processed sidechain buffer
                        .zip(&self.sidechain_complex_buffer[channel_index])
                    {
                        let (frequency_magnitude, phase) = main_bin.to_polar();
                        let sidechain_frequency_magnitude = sidechain_bin.norm();

                        let result_magnitude = clamp_min(
                            frequency_magnitude - (sidechain_frequency_magnitude * self.overlap_times as f32),
                            0.0,
                        );

                        // Reconstruct the complex value from the main input's phase and our output magnitude
                        *main_bin = Complex32::from_polar(result_magnitude, phase);
                    }

                    if let Some(output_buffer_in) = self.output_buffer_in.as_mut() {
                        let buffer = output_buffer_in.input_buffer();
                        buffer[channel_index].copy_from_slice(self.main_complex_buffer.as_slice());

                        // publish if both channels have been supplied
                        if channel_index == CHANNELS - 1 {
                            output_buffer_in.publish();
                        }
                    }

                    // Set the imaginary values of the first and last frequency bins to zero
                    // so the inverse FFT can be performed
                    self.main_complex_buffer.first_mut().unwrap().im.set_zero();
                    self.main_complex_buffer.last_mut().unwrap().im.set_zero();

                    self.inverse_fft.process(&mut self.main_complex_buffer, real_buffer);
                    
                    // apply gain compensation
                    real_buffer.iter_mut().for_each(|sample| *sample *= self.cola / WINDOW_SIZE as f32);
                }
            },
        );
    }

    pub fn latency_samples(&self) -> u32 {
        self.stft.latency_samples()
    }

    pub fn reset(&mut self) {
        self.stft.set_block_size(WINDOW_SIZE);
    }

    pub fn set_input_buffer_in(&mut self, buffer_in: Option<triple_buffer::Input<[[Complex32; SPECTRUM_SIZE]; CHANNELS]>>) {
        self.input_buffer_in = buffer_in;
    }

    pub fn set_sidechain_buffer_in(&mut self, buffer_in: Option<triple_buffer::Input<[[Complex32; SPECTRUM_SIZE]; CHANNELS]>>) {
        self.sidechain_buffer_in = buffer_in;
    }

    pub fn set_output_buffer_in(&mut self, buffer_in: Option<triple_buffer::Input<[[Complex32; SPECTRUM_SIZE]; CHANNELS]>>) {
        self.output_buffer_in = buffer_in;
    }
}

impl<const CHANNELS: usize, const WINDOW_SIZE: usize, const HOP_SIZE: usize, const SPECTRUM_SIZE: usize> Default for FrequencySidechain<CHANNELS, WINDOW_SIZE, HOP_SIZE, SPECTRUM_SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

fn detail_smoothing(detail: f32, frequency_bins: &mut [Complex<f32>], sample_rate: f32) {
    if detail == 1.0 { return; }

    let nyquist = sample_rate / 2.0;
    let cutoff_frequency = detail.max(f32::MIN_POSITIVE) * nyquist;  // Calculate the cutoff frequency proportional to detail
    let num_bins = frequency_bins.len();
    for (i, bin) in frequency_bins.iter_mut().enumerate() {
        let magnitude = bin.norm();
        if magnitude == 0.0 { continue; }

        let frequency = (i as f32 / num_bins as f32) * nyquist;

        // Create a low-pass filter with a Gaussian shape
        let filter = (-(frequency / cutoff_frequency).powi(2)).exp();

        // Apply the filter to the frequency bins
        *bin = Complex32::from_polar(magnitude * filter, bin.arg());
    }
}

fn precision_scaling(precision: f32, frequency_bins: &mut [Complex<f32>]) {
    if precision == 0.0 { return; }

    let alpha = if precision > 0.0 {
        1.0 / (1.0 - precision)
    } else {
        1.0 - (precision.abs() * 0.999)
    };

    let mut magnitudes = frequency_bins.iter().map(|bin| bin.norm()).collect::<Vec<_>>();

    let (mut min_magnitude, mut max_magnitude) = (f32::INFINITY, 0_f32);
    let (mut min_scaled_magnitude, mut max_scaled_magnitude) = (f32::INFINITY, 0_f32);

    for magnitude in magnitudes.iter_mut() {
        min_magnitude = min_magnitude.min(*magnitude);
        max_magnitude = max_magnitude.max(*magnitude);

        *magnitude = magnitude.powf(alpha).max(0.0);

        min_scaled_magnitude = min_scaled_magnitude.min(*magnitude);
        max_scaled_magnitude = max_scaled_magnitude.max(*magnitude);
    }

    for (bin, magnitude) in frequency_bins.iter_mut().zip(magnitudes.iter()) {
        let scaled_magnitude = (magnitude - min_scaled_magnitude) / (max_scaled_magnitude - min_scaled_magnitude)
            * (max_magnitude - min_magnitude) + min_magnitude;
        *bin = Complex32::from_polar(scaled_magnitude.max(0.0), bin.arg());
    }
}

impl Default for FrequencySidechainParams {
    fn default() -> Self {
        Self {
            detail: FloatParam::new(
                "Detail",
                1.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 1.0,
                    factor: FloatRange::skew_factor(-2.0)
                }
            )
                .with_unit("%")
                .with_value_to_string(formatters::v2s_f32_percentage(0))
                .with_string_to_value(formatters::s2v_f32_percentage()),
            precision: FloatParam::new("Precision", 0.0, FloatRange::Linear { min: -1.0, max: 1.0 })
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
        let mut fs = FrequencySidechain::<2, 1024, 128, 513>::new();
        let mut main_buffer = create_empty_buffer(1024);
        let mut sidechain_buffer = create_empty_buffer(1024);
        fs.process(&mut main_buffer, &mut sidechain_buffer, SAMPLE_RATE, &FrequencySidechainParams::default());
        main_buffer.on_each(|channel, sample_index, sample| {
            assert_eq!(*sample, 0.0, "Channel {} | Sample {}", channel, sample_index);
        });
    }

    #[test]
    fn test_equal_buffers() {
        let mut fs = FrequencySidechain::<2, 1024, 128, 513>::new();
        let frequency = 440_f32;
        let mut main_buffer = create_buffer_with(1024, |_, sample_index| {
            ((frequency / SAMPLE_RATE) * sample_index as f32).sin()
        });
        let mut sidechain_buffer = create_buffer_with(1024, |_, sample_index| {
            ((frequency / SAMPLE_RATE) * sample_index as f32).sin()
        });
        fs.process(&mut main_buffer, &mut sidechain_buffer, SAMPLE_RATE, &FrequencySidechainParams::default());
        main_buffer.on_each(|channel, sample_index, sample| {
            assert_eq!(*sample, 0.0, "Channel {} | Sample {}", channel, sample_index);
        });
    }
    
    #[test]
    fn test_remove_1_of_2_frequencies() {
        let mut fs = FrequencySidechain::<2, 1024, 128, 513>::new();
        let frequency_1 = 440_f32;
        let frequency_2 = 880_f32;
        let mut main_buffer = create_buffer_with(1024, |_, sample_index| {
            ((frequency_1 / SAMPLE_RATE) * sample_index as f32).sin() + ((frequency_2 / SAMPLE_RATE) * sample_index as f32).sin()
        });
        let mut sidechain_buffer = create_buffer_with(1024, |_, sample_index| {
            ((frequency_2 / SAMPLE_RATE) * sample_index as f32).sin()
        });
        fs.process(&mut main_buffer, &mut sidechain_buffer, SAMPLE_RATE, &FrequencySidechainParams::default());
        main_buffer.on_each(|channel, sample_index, sample| {
            assert_eq!(*sample, 0.0, "Channel {} | Sample {}", channel, sample_index);
            // assert_eq!(*sample, ((frequency_1 / SAMPLE_RATE) * sample_index as f32).sin(), "Channel {} | Sample {}", channel, sample_index);
        });
    }
}