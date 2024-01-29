#[macro_use]
extern crate nih_plug;

pub use freqchain::FreqChain;

pub mod audio_processing;
pub mod freqchain;
pub mod params;
pub mod plugin_formats;
pub mod ui;

nih_export_clap!(FreqChain);
nih_export_vst3!(FreqChain);
