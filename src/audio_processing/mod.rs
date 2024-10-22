pub(crate) mod fft;
pub(crate) mod biquad_filter;
pub mod frequency_sidechain;
pub mod equalizer;
pub mod spectrum;
pub mod smoother;
mod util;

const DEFAULT_CHANNELS: usize = 2;
