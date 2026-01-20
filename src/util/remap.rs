use realfft::num_traits::Float;
use std::ops::{Add, Bound, Div, Mul, RangeBounds, Sub};

#[inline]
pub fn normalize<S>(value: S, min: S, max: S) -> S
where
    S: Sub<Output = S> + Div<Output = S> + Copy
{
    (value - min) / (max - min)
}

#[inline]
pub fn normalize_log10<S: Float>(value: S, min: S, max: S) -> S {
    (value / min).log10() / (max / min).log10()
}

#[inline]
pub fn normalize_ranged<S, R>(value: S, range: &R) -> S
where
    S: Sub<Output = S> + Div<Output = S> + Copy,
    R: RangeBounds<S>,
{
    let (min, max) = range_bounds(range);
    normalize(value, min, max)
}

#[inline]
pub fn normalize_log10_ranged<S: Float, R: RangeBounds<S>>(value: S, range: &R) -> S {
    let (min, max) = range_bounds(range);
    normalize_log10(value, min, max)
}

#[inline]
pub fn map_normalized<S>(normalized_value: S, target_min: S, target_max: S) -> S
where
    S: Add<Output = S> + Sub<Output = S> + Mul<Output = S> + Copy
{
    target_min + normalized_value * (target_max - target_min)
}

#[inline]
pub fn map_normalized_log10<S: Float>(normalized_value: S, target_min: S, target_max: S) -> S {
    target_min * S::from(10).unwrap().powf(normalized_value * (target_max / target_min).log10())
}

#[inline]
pub fn map_normalized_ranged<S, R>(normalized_value: S, target_range: &R) -> S
where
    S: Add<Output = S> + Sub<Output = S> + Mul<Output = S> + Copy,
    R: RangeBounds<S>,
{
    let (min, max) = range_bounds(target_range);
    map_normalized(normalized_value, min, max)
}

#[inline]
pub fn map_normalized_log10_ranged<S: Float, R: RangeBounds<S>>(normalized_value: S, target_range: &R) -> S {
    let (min, max) = range_bounds(target_range);
    map_normalized_log10(normalized_value, min, max)
}

#[inline]
fn range_bounds<S: Copy, R: RangeBounds<S>>(range: &R) -> (S, S) {
    let start = match range.start_bound() {
        Bound::Included(&val) | Bound::Excluded(&val) => val,
        Bound::Unbounded => panic!("Range start must be bounded"),
    };
    let end = match range.end_bound() {
        Bound::Included(&val) | Bound::Excluded(&val) => val,
        Bound::Unbounded => panic!("Range end must be bounded"),
    };
    (start, end)
}
