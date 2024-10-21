use std::sync::Arc;

use nih_plug::prelude::*;
use nih_plug_iced::IcedState;

use crate::{
    audio_processing::{
        equalizer::{EqualizerParams},
        frequency_sidechain::FrequencySidechainParams,
    },
    freqchain::EQ_BAND_COUNT,
    ui::editor,
};
use crate::audio_processing::smoother::SmootherParams;

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

impl FreqChainParams {
    pub fn new() -> Self {
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