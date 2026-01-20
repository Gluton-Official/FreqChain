use nih_plug::prelude::FloatRange;
use nih_plug::util;

pub mod fft;
pub mod buffer_utils;
pub mod biquad_filter;
pub mod remap;

#[inline]
pub fn gain_range_from_db(min_db: f32, max_db: f32) -> FloatRange {
    FloatRange::Skewed {
        min: util::db_to_gain(min_db),
        max: util::db_to_gain(max_db),
        // skew the float gain value to be linear decibels
        factor: FloatRange::gain_skew_factor(min_db, max_db)
    }
}