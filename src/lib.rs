#[macro_use]
extern crate nih_plug;

pub use freqchain::FreqChain;

pub mod modules;
pub mod freqchain;
pub mod plugin_formats;
pub mod ui;
pub mod util;

nih_export_clap!(FreqChain);
nih_export_vst3!(FreqChain);
