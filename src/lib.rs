use std::sync::Arc;
use std::sync::atomic::Ordering;
use nih_plug::prelude::*;
use nih_plug_iced::IcedState;

mod editor;

const PEAK_METER_DECAY_MS: f64 = 150.0;

struct FreqChain {
    params: Arc<FreqChainParams>,

    /// Used to normalize the meter's response based on sample rate
    peak_meter_decay_weight: f32,
    /// Current voltage gain of the peak meter
    peak_meter: Arc<AtomicF32>,
}

/// The [`Params`] derive macro provides the plugin wrapper (i.e. within a DAW) the plugin's
/// parameters, persistent serializable fields, and nested parameter groups.
#[derive(Params)]
struct FreqChainParams {
    #[persist = "editor-state"]
    editor_state: Arc<IcedState>,

    #[id = "gain"]
    pub gain: FloatParam,
}

impl Default for FreqChain {
    fn default() -> Self {
        Self {
            params: Arc::new(FreqChainParams::default()),

            peak_meter_decay_weight: 1.0,
            peak_meter: Arc::new(AtomicF32::new(util::MINUS_INFINITY_DB)),
        }
    }
}

impl Default for FreqChainParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),

            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(0.0),
                // Set the gain range from -30 dB to 30 dB, skewing the float gain value to be
                // linear decibels
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
                // Smooth the gain parameter logarithmically because it is in linear gain
                .with_smoother(SmoothingStyle::Logarithmic(50.0))
                .with_unit(" dB") // Unit suffix to parameter display
                // Set value transformers for display
                .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
                .with_string_to_value(formatters::s2v_f32_gain_to_db()),
        }
    }
}

impl Plugin for FreqChain {
    const NAME: &'static str = "FreqChain";
    const VENDOR: &'static str = "Gluton";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "glutonofficial@gmail.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    /// Add [`AuxiliaryBuffers`] for sidechain signal
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),

            aux_input_ports: &[new_nonzero_u32(2)],
            aux_output_ports: &[],

            names: PortNames::const_default(),
        },
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
            self.peak_meter.clone(),
            self.params.editor_state.clone(),
        )
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // After `PEAK_METER_DECAY_MS` milliseconds of silence, the peak meter's value should drop by 12 dB
        // TODO: figure out values
        self.peak_meter_decay_weight = 0.25f64
            .powf((buffer_config.sample_rate as f64 * PEAK_METER_DECAY_MS / 1000.0).recip())
            as f32;

        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>
    ) -> ProcessStatus {
        for channel_samples in buffer.iter_samples() {
            let mut amplitude = 0.0;
            let num_samples = channel_samples.len();

            // Parameters have smoothing built in
            let gain = self.params.gain.smoothed.next();
            
            for sample in channel_samples {
                *sample *= gain;
                amplitude += *sample;
            }

            // Only update meter when editor GUI is open
            if self.params.editor_state.is_open() {
                // TODO: comment
                amplitude = (amplitude / num_samples as f32).abs();
                let current_peak_meter = self.peak_meter.load(Ordering::Relaxed);
                let new_peak_meter = if amplitude > current_peak_meter {
                    amplitude
                } else {
                    current_peak_meter * self.peak_meter_decay_weight
                        + amplitude * (1.0 - self.peak_meter_decay_weight)
                };

                self.peak_meter
                    .store(new_peak_meter, Ordering::Relaxed);
            }
        }

        ProcessStatus::Normal // allow for suspense if no input audio
    }
}

impl ClapPlugin for FreqChain {
    const CLAP_ID: &'static str = "dev.gluton.freqchain";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Frequency Sidechaining Plugin");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    /// TODO: use [`ClapFeature::Gate`]
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo, ClapFeature::Mono];
}

impl Vst3Plugin for FreqChain {
    const VST3_CLASS_ID: [u8; 16] = *b"gluton_freqchain";

    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Dynamics];
}

nih_export_clap!(FreqChain);
nih_export_vst3!(FreqChain);