use nih_plug::prelude::FloatRange;
use nih_plug::util;

pub mod fft;
pub mod buffer_utils;
pub mod biquad_filter;
pub mod remap;

#[inline]
pub fn gain_range_from_db(min_db: f32, max_db: f32, center_db: f32) -> FloatRange {
    let min_gain = util::db_to_gain(min_db);
    let max_gain = util::db_to_gain(max_db);
    let center_gain = util::db_to_gain(center_db);

    // Check the Skewed equation in the normalized function below, we need to solve the factor
    // such that the a normalized value of 0.5 resolves to the middle of the range
    FloatRange::Skewed {
        min: min_gain,
        max: max_gain,
        factor: 0.5f32.log((center_gain - min_gain) / (max_gain - min_gain))
    }
}