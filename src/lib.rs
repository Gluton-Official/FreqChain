use std::error::Error;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::atomic::Ordering;

use nih_plug::prelude::*;
use nih_plug_iced::IcedState;
use realfft::{ComplexToReal, RealFftPlanner, RealToComplex};
use realfft::num_complex::{Complex, ComplexFloat};

mod editor;
mod widgets {
    pub mod param_knob;
}

const PEAK_METER_DECAY_MS: f32 = 150.0;

pub struct FreqChain {
    params: Arc<FreqChainParams>,

    fft_plan: Option<Arc<dyn RealToComplex<f32>>>,
    sidechain_fft_plan: Option<Arc<dyn RealToComplex<f32>>>,
    fft_inverse_plan: Option<Arc<dyn ComplexToReal<f32>>>,

    /// The weight to apply to the previous peak meter value when calculating the new peak meter value
    peak_meter_decay_weight: f32,
    input_peak_meter_value: Arc<AtomicF32>,
    output_peak_meter_value: Arc<AtomicF32>,
    sidechain_input_peak_meter_value: Arc<AtomicF32>,
    sidechain_output_peak_meter_value: Arc<AtomicF32>,
}

/// The [`Params`] derive macro provides the plugin wrapper (e.g. within a DAW) the plugin's
/// parameters, persistent serializable fields, and nested parameter groups.
#[derive(Params)]
struct FreqChainParams {
    #[persist = "editor-state"]
    editor_state: Arc<IcedState>,

    #[id = "sidechain-gain"]
    pub sidechain_gain: FloatParam,
    #[id = "detail"]
    pub detail: FloatParam,

    #[id = "is-mono"]
    pub is_mono: BoolParam,
}

impl Default for FreqChain {
    fn default() -> Self {
        Self {
            params: Arc::new(FreqChainParams::default()),

            fft_plan: None,
            sidechain_fft_plan: None,
            fft_inverse_plan: None,

            peak_meter_decay_weight: 1.0,
            input_peak_meter_value: Arc::new(AtomicF32::new(util::MINUS_INFINITY_DB)),
            output_peak_meter_value: Arc::new(AtomicF32::new(util::MINUS_INFINITY_DB)),
            sidechain_input_peak_meter_value: Arc::new(AtomicF32::new(util::MINUS_INFINITY_DB)),
            sidechain_output_peak_meter_value: Arc::new(AtomicF32::new(util::MINUS_INFINITY_DB)),
        }
    }
}

impl Default for FreqChainParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),

            sidechain_gain: FloatParam::new(
                "Gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(util::MINUS_INFINITY_DB),
                    max: util::db_to_gain(24.0),
                    // skew the float gain value to be linear decibels
                    factor: FloatRange::gain_skew_factor(util::MINUS_INFINITY_DB, 24.0),
                },
            )
                // Smooth the gain parameter logarithmically because it is in linear gain
                // TODO: test w/ & w/o smoothing
                .with_smoother(SmoothingStyle::Logarithmic(50.0))
                .with_unit("dB") // Unit suffix to parameter display
                // Set value transformers for display
                .with_value_to_string(formatters::v2s_f32_gain_to_db(1))
                .with_string_to_value(formatters::s2v_f32_gain_to_db()),

            detail: FloatParam::new(
                "Detail",
                0.5,
                FloatRange::Linear {
                    min: 0.0,
                    max: 1.0,
                },
            )
                .with_unit("%")
                .with_value_to_string(formatters::v2s_f32_percentage(0))
                .with_string_to_value(formatters::s2v_f32_percentage()),

            is_mono: BoolParam::new("Mono", false),
        }
    }
}

impl Plugin for FreqChain {
    const NAME: &'static str = "FreqChain";
    const VENDOR: &'static str = "Gluton";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "glutonofficial@gmail.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        // Stereo layout
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),

            aux_input_ports: &[new_nonzero_u32(2)],
            aux_output_ports: &[],

            names: PortNames::const_default(),
        },
        // Mono layout
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),

            aux_input_ports: &[new_nonzero_u32(1)],
            aux_output_ports: &[],

            names: PortNames::const_default(),
        }
    ];

    /// Tell the wrapper to split the audio buffer into smaller blocks when there are inter-buffer
    /// parameter changes, allowing the wrapper to handle transport and timing information between
    /// splits.
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    // Do not support sending/receiving SysEx MIDI messages
    type SysExMessage = ();
    // No background tasks
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(
            self.params.clone(),
            self.input_peak_meter_value.clone(),
            self.params.editor_state.clone(),
        )
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        context: &mut impl InitContext<Self>,
    ) -> bool {
        // After `PEAK_METER_DECAY_MS` milliseconds of silence, the peak meter's value should drop by 12 dB
        // TODO: figure out values
        self.peak_meter_decay_weight = 0.25f32
            .powf((buffer_config.sample_rate * PEAK_METER_DECAY_MS / 1000.0).recip());

        let mut fft_planner = RealFftPlanner::<f32>::new();
        let mut sidechain_fft_planner = RealFftPlanner::<f32>::new();

        // Use the closest power of 2 equal or greater than the max buffer size
        if let Some(fft_size) = buffer_config.max_buffer_size.checked_next_power_of_two() {
            self.fft_plan = Some(fft_planner.plan_fft_forward(fft_size as usize));
            self.sidechain_fft_plan = Some(sidechain_fft_planner.plan_fft_forward(fft_size as usize));
            self.fft_inverse_plan = Some(fft_planner.plan_fft_inverse(fft_size as usize));

            // set latency to size of the FFT
            context.set_latency_samples(fft_size);

            true
        } else {
            nih_log!("Buffer size is too large for FFT");
            false
        }
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>
    ) -> ProcessStatus {
        let sidechain_buffer = &mut aux.inputs[0];

        nih_debug_assert_eq!(buffer.channels(), sidechain_buffer.channels(), "Sidechain buffer channels does not equal main buffer channels");
        nih_debug_assert_eq!(buffer.samples(), sidechain_buffer.samples(), "Sidechain buffer samples does not equal main buffer samples");

        let channels = if self.params.is_mono.value() { 1 } else { buffer.channels() };

        // Create an input and output vector per channel for the FFT
        let mut sidechain_fft_input: Vec<Vec<f32>> = (0..channels)
            .map(|_| self.sidechain_fft_plan.as_mut().unwrap().make_input_vec()).collect();
        let mut sidechain_spectrum: Vec<Vec<Complex<f32>>> = (0..channels)
            .map(|_| self.sidechain_fft_plan.as_mut().unwrap().make_output_vec()).collect();

        // Read sidechain buffer
        for (sample_index, sidechain_channel_samples) in sidechain_buffer.iter_samples().enumerate() {
            let mut sidechain_input_amplitude = 0.0;
            let mut sidechain_output_amplitude = 0.0;

            // Get the current interpolated gain value
            let gain = self.params.sidechain_gain.smoothed.next();

            // For each channel's sample
            for (channel_number, channel_sample) in sidechain_channel_samples.into_iter().enumerate() {
                // Sum pre-processed amplitude
                sidechain_input_amplitude += *channel_sample;

                // Processing
                *channel_sample *= gain;

                // Sum post-processed amplitude
                sidechain_output_amplitude += *channel_sample;

                // Copy to FFT input
                sidechain_fft_input[channel_number][sample_index] = *channel_sample;
            }

            // Only update meter when editor GUI is open
            if self.params.editor_state.is_open() {
                // Calculate the average amplitude of the the sample in each of the channels
                sidechain_input_amplitude = (sidechain_input_amplitude / channels as f32).abs();
                sidechain_output_amplitude = (sidechain_output_amplitude / channels as f32).abs();

                self.update_peak_meter(&self.sidechain_input_peak_meter_value, sidechain_input_amplitude);
                self.update_peak_meter(&self.sidechain_output_peak_meter_value, sidechain_output_amplitude);
            }
        }

        // Compute the FFT for each of the sidechain channels
        for (channel_number, sidechain_fft_channel_input) in sidechain_fft_input.iter_mut().enumerate() {
            if let Err(e) = self.sidechain_fft_plan.as_mut().unwrap().process(sidechain_fft_channel_input, &mut sidechain_spectrum[channel_number]) {
                nih_log!("Sidechain FFT failed: {e}");
                return ProcessStatus::Error("Sidechain FFT failed");
            }
        }

        // TODO: update sidechain signal histogram here

        // Create an input and output vector per channel for the FFT
        let mut fft_input: Vec<Vec<f32>> = (0..channels)
            .map(|_| self.fft_plan.as_mut().unwrap().make_input_vec()).collect();
        let mut spectrum: Vec<Vec<Complex<f32>>> = (0..channels)
            .map(|_| self.fft_plan.as_mut().unwrap().make_output_vec()).collect();

        // Read input buffer
        for (sample_index, channel_samples) in buffer.iter_samples().enumerate() {
            let mut input_amplitude = 0.0;

            // For each channel's sample
            for (channel_number, channel_sample) in channel_samples.into_iter().enumerate() {
                // Sum amplitude
                input_amplitude += *channel_sample;

                // Copy to FFT input
                fft_input[channel_number][sample_index] = *channel_sample;
            }

            // Only update meter when editor GUI is open
            if self.params.editor_state.is_open() {
                // Calculate the average amplitude of the channel
                input_amplitude = (input_amplitude / channels as f32).abs();

                self.update_peak_meter(&self.input_peak_meter_value, input_amplitude);
            }
        }

        // Compute the FFT for each of the channels
        for (channel, fft_channel_input) in fft_input.iter_mut().enumerate() {
            if let Err(e) = self.fft_plan.as_mut().unwrap().process(fft_channel_input, &mut spectrum[channel]) {
                nih_log!("FFT failed: {e}");
                return ProcessStatus::Error("FFT failed");
            }
        }

        // TODO: update input signal histogram here

        // Subtract the magnitudes of the input spectrum by the sidechain spectrum, retaining the input phase
        let mut result_spectrum: Vec<Vec<Complex<f32>>> =
            spectrum.iter().zip(sidechain_spectrum).map(|(channel, sidechain_channel)| {
                channel.iter().zip(sidechain_channel).map(|(sample, sidechain_sample)| {
                    let sample_magnitude = sample.abs();
                    let sidechain_sample_magnitude = sidechain_sample.abs();

                    let magnitude = std::cmp::max(sample_magnitude - sidechain_sample_magnitude, 0.0);
                    let phase = sample.arg();

                    magnitude * Complex::cis(phase)
                }).collect()
            }).collect();

        // TODO: update output signal histogram here

        // Create an output vector per channel for the inverse FFT
        let mut fft_inverse_output: Vec<Vec<f32>> = (0..channels)
            .map(|_| self.fft_inverse_plan.as_mut().unwrap().make_output_vec()).collect();

        // Compute the inverse FFT for each of the channels
        for (channel, result_spectrum_channel) in result_spectrum.iter_mut().enumerate() {
            if let Err(e) = self.fft_inverse_plan.as_mut().unwrap().process(result_spectrum_channel, &mut fft_inverse_output[channel]) {
                nih_log!("FFT inverse failed: {e}");
                return ProcessStatus::Error("FFT inverse failed");
            }
        }

        // Copy the inverse FFT output to the buffer
        for (sample_index, channel_samples) in buffer.iter_samples().enumerate() {
            for (channel_number, channel_sample) in channel_samples.into_iter().enumerate() {
                *channel_sample = fft_inverse_output[channel_number][sample_index];
            }
        }

        // TODO: update output peak meter here

        ProcessStatus::Normal // allow for suspense if no input audio
    }
}

impl FreqChain {
    fn update_peak_meter(&self, peak_meter_value: &Arc<AtomicF32>, amplitude: f32) {
        let current_peak_meter_value = peak_meter_value.load(Ordering::Relaxed);
        // Increase the peak meter value or gradually decay it
        let new_peak_meter_value = if amplitude > current_peak_meter_value {
            amplitude
        } else {
            current_peak_meter_value * self.peak_meter_decay_weight
                + amplitude * (1.0 - self.peak_meter_decay_weight)
        };

        peak_meter_value.store(new_peak_meter_value, Ordering::Relaxed);
    }
}

impl ClapPlugin for FreqChain {
    const CLAP_ID: &'static str = "dev.gluton.freqchain";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Frequency Sidechaining Plugin");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Mono,
        ClapFeature::Gate,
    ];
}

impl Vst3Plugin for FreqChain {
    const VST3_CLASS_ID: [u8; 16] = *b"gluton_freqchain";

    // TODO: add Stereo and Mono categories?
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Fx,
        Vst3SubCategory::Dynamics,
    ];
}

nih_export_clap!(FreqChain);
nih_export_vst3!(FreqChain);