use nih_plug::{
    plugin::Plugin,
    prelude::{ClapFeature, ClapPlugin},
};

use crate::freqchain::FreqChain;

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
