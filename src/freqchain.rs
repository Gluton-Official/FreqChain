use std::num::NonZeroU32;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;

use nih_plug::prelude::*;
use nih_plug_iced::IcedState;

use crate::modules::equalizer::Equalizer;
use crate::modules::equalizer::EqualizerParams;
use crate::modules::frequency_sidechain::FrequencySidechain;
use crate::modules::frequency_sidechain::FrequencySidechainParams;
use crate::modules::spectrum::Spectrum;
use crate::modules::spectrum::SpectrumOutput;
use crate::ui::editor;
use crate::util::buffer_utils::BufferUtils;

const SMOOTHING_DECAY_MS: f32 = 150.0;

const CHANNELS: usize = 2;

const EQ_BAND_COUNT: usize = 7;

const FFT_WINDOW_SIZE: usize = 1024;
const FFT_HOP_SIZE: usize = 128;

pub struct FreqChain {
    params: Arc<FreqChainParams>,

    sample_rate: Arc<AtomicF32>,

    /// The weight to apply to the previous peak values when calculating the new peak value,
    /// Usable for peak meter, spectrum, etc.
    smoothing_decay_weight: f32,

    input_peak_meter_value: Arc<AtomicF32>,
    output_peak_meter_value: Arc<AtomicF32>,
    sidechain_input_peak_meter_value: Arc<AtomicF32>,
    sidechain_output_peak_meter_value: Arc<AtomicF32>,

    equalizer: Equalizer<EQ_BAND_COUNT, CHANNELS>,

    sidechain_spectrum: Spectrum,
    sidechain_spectrum_output: Arc<Mutex<SpectrumOutput>>,

    frequency_sidechain: FrequencySidechain,
}

/// The [`Params`] derive macro provides the plugin wrapper (e.g. within a DAW) the plugin's
/// parameters, persistent serializable fields, and nested parameter groups.
#[derive(Params)]
pub struct FreqChainParams {
    #[persist = "editor_state"]
    pub editor_state: Arc<IcedState>,

    #[id = "mono_processing"]
    pub mono_processing: BoolParam,

    #[nested(id_prefix = "sidechain_input", group = "Sidechain Input")]
    pub sidechain_input: SidechainInputParams,

    #[nested(id_prefix = "equalizer", group = "Equalizer")]
    pub equalizer: EqualizerParams<EQ_BAND_COUNT>,
    #[nested(id_prefix = "frequency_sidechain", group = "Frequency Sidechain")]
    pub frequency_sidechain: FrequencySidechainParams,
}

#[derive(Params)]
pub struct SidechainInputParams {
    #[id = "solo"]
    pub solo: BoolParam,
    #[id = "gain"]
    pub gain: FloatParam,
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
            main_input_channels: NonZeroU32::new(CHANNELS as u32),
            main_output_channels: NonZeroU32::new(CHANNELS as u32),

            aux_input_ports: &[new_nonzero_u32(CHANNELS as u32)],

            ..AudioIOLayout::const_default()
        },
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
            self.sample_rate.clone(),
            self.input_peak_meter_value.clone(),
            self.sidechain_spectrum_output.clone(),
            self.params.editor_state.clone(),
        )
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        context: &mut impl InitContext<Self>,
    ) -> bool {
        self.sample_rate.store(buffer_config.sample_rate, Ordering::Relaxed);

        self.equalizer.set_sample_rate(buffer_config.sample_rate);

        // After `SMOOTHING_DECAY_MS` milliseconds of silence, the peak meter's value should drop by 12 dB
        self.smoothing_decay_weight = 0.25f32.powf((buffer_config.sample_rate * SMOOTHING_DECAY_MS / 1000.0).recip());

        self.sidechain_spectrum
            .set_smoothing_decay_weight(self.smoothing_decay_weight);

        context.set_latency_samples(self.frequency_sidechain.latency_samples());

        true
    }

    fn reset(&mut self) {
        self.frequency_sidechain.reset()
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let sidechain_buffer = &mut aux.inputs[0];

        if self.params.mono_processing.value() && sidechain_buffer.channels() != 1 {
            let channels = sidechain_buffer.channels() as f32;
            for mut sidechain_channel_samples in sidechain_buffer.iter_samples() {
                let averaged_sample = sidechain_channel_samples.iter_mut().map(|x| *x / channels).sum();
                for sidechain_sample in sidechain_channel_samples {
                    *sidechain_sample = averaged_sample;
                }
            }
        }

        sidechain_buffer.on_each_sample(|_, _, sidechain_sample| {
            *sidechain_sample *= self.params.sidechain_input.gain.smoothed.next();
        });

        self.equalizer.process(sidechain_buffer, &self.params.equalizer);

        if self.params.sidechain_input.solo.value() {
            for (channel_samples, sidechain_channel_samples) in buffer.iter_samples().zip(sidechain_buffer.iter_samples()) {
                for (sample, sidechain_sample) in channel_samples.into_iter().zip(sidechain_channel_samples) {
                    *sample = *sidechain_sample;
                }
            }
            return ProcessStatus::Normal;
        }

        self.frequency_sidechain.process(
            buffer,
            sidechain_buffer,
            self.sample_rate.load(Ordering::Relaxed),
            &self.params.frequency_sidechain,
        );

        ProcessStatus::Normal // allow for suspense if no input audio
    }
}

impl Default for FreqChain {
    fn default() -> Self {
        let (sidechain_spectrum, sidechain_spectrum_output) = Spectrum::new();

        Self {
            params: Arc::new(FreqChainParams::default()),

            sample_rate: Arc::new(AtomicF32::new(1.0)),

            smoothing_decay_weight: 1.0,

            input_peak_meter_value: Arc::new(AtomicF32::new(util::MINUS_INFINITY_DB)),
            output_peak_meter_value: Arc::new(AtomicF32::new(util::MINUS_INFINITY_DB)),
            sidechain_input_peak_meter_value: Arc::new(AtomicF32::new(util::MINUS_INFINITY_DB)),
            sidechain_output_peak_meter_value: Arc::new(AtomicF32::new(util::MINUS_INFINITY_DB)),

            equalizer: Equalizer::<EQ_BAND_COUNT, CHANNELS>::default(),

            sidechain_spectrum,
            sidechain_spectrum_output: Arc::new(Mutex::new(sidechain_spectrum_output)),

            frequency_sidechain: FrequencySidechain::new(CHANNELS, FFT_WINDOW_SIZE, FFT_HOP_SIZE),
        }
    }
}

impl Default for FreqChainParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),

            mono_processing: BoolParam::new("Mono Processing", false),

            sidechain_input: SidechainInputParams::default(),

            equalizer: EqualizerParams::default(),
            frequency_sidechain: FrequencySidechainParams::default(),
        }
    }
}

impl Default for SidechainInputParams {
    fn default() -> Self {
        Self {
            solo: BoolParam::new("Sidechain Input Solo", false),
            gain: FloatParam::new(
                "Sidechain Input Gain",
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
        }
    }
}
