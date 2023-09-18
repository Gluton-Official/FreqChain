use std::sync::Arc;

use nih_plug::prelude::*;

struct FreqChain {
    params: Arc<FreqChainParams>,
}

/// The [`Params`] derive macro provides the plugin wrapper (i.e. within a DAW) the plugin's
/// parameters, persistent serializable fields, and nested parameter groups.
#[derive(Params)]
struct FreqChainParams {
    #[id = "gain"]
    pub gain: FloatParam,
}

impl Default for FreqChain {
    fn default() -> Self {
        Self {
            params: Arc::new(FreqChainParams::default()),
        }
    }
}

impl Default for FreqChainParams {
    fn default() -> Self {
        Self {
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
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),

        aux_input_ports: &[],
        aux_output_ports: &[],

        names: PortNames::const_default(),
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

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

    /// Optional, [`FreqChain::reset`] is called right after this function.
    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // TODO: resize buffers / perform expensive initialization operations
        true
    }

    /// Optional, can be called from audio thread and may not allocate
    fn reset(&mut self) {
        // TODO: reset buffers and envelopes
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>
    ) -> ProcessStatus {
        for channel_samples in buffer.iter_samples() {
            // Parameters have smoothing built in
            let gain = self.params.gain.smoothed.next();
            
            for sample in channel_samples {
                *sample *= gain;
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
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

impl Vst3Plugin for FreqChain {
    const VST3_CLASS_ID: [u8; 16] =*b"gluton_freqchain";

    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Dynamics];
}

nih_export_clap!(FreqChain);
nih_export_vst3!(FreqChain);