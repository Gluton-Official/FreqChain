use std::sync::Arc;

use nih_plug::params::{BoolParam, FloatParam, Params};
use nih_plug::prelude::{FloatRange, SmoothingStyle};
use nih_plug::{formatters, util};
use nih_plug::wrapper::state::ParamValue::Bool;
use nih_plug_iced::IcedState;
use crate::audio_processing::equalizer::Equalizer;

use crate::audio_processing::equalizer::EqualizerParams;
use crate::ui::editor;
use crate::freqchain::EQ_BAND_COUNT;

/// The [`Params`] derive macro provides the plugin wrapper (e.g. within a DAW) the plugin's
/// parameters, persistent serializable fields, and nested parameter groups.
#[derive(Params)]
pub struct FreqChainParams {
    #[persist = "editor_state"]
    pub editor_state: Arc<IcedState>,

    #[id = "sidechain_gain"]
    pub sidechain_gain: FloatParam,
    #[id = "detail"]
    pub detail: FloatParam,

    #[id = "is_mono"]
    pub is_mono: BoolParam,

    #[nested(id_prefix = "equalizer", group = "equalizer")]
    pub equalizer: EqualizerParams<EQ_BAND_COUNT>,

    #[nested(id_prefix = "debug", group = "debug")]
    pub debug: DebugParams,
}

#[derive(Params)]
pub struct DebugParams {
    #[id = "output_sidechain_only"]
    pub output_sidechain_only: BoolParam,
}

impl FreqChainParams {
    pub fn new() -> Self {
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

            detail: FloatParam::new("Detail", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_unit("%")
                .with_value_to_string(formatters::v2s_f32_percentage(0))
                .with_string_to_value(formatters::s2v_f32_percentage()),

            is_mono: BoolParam::new("Mono", false),

            equalizer: EqualizerParams::default(),

            debug: DebugParams::default(),
        }
    }
}

impl Default for DebugParams {
    fn default() -> Self {
        Self {
            output_sidechain_only: BoolParam::new("Output Sidechain Only", false),
        }
    }
}