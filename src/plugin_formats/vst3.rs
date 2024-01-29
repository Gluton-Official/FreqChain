use nih_plug::prelude::{Vst3Plugin, Vst3SubCategory};

use crate::freqchain::FreqChain;

impl Vst3Plugin for FreqChain {
    const VST3_CLASS_ID: [u8; 16] = *b"gluton_freqchain";

    // TODO: add Stereo and Mono categories?
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Dynamics];
}
