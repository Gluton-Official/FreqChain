
#[derive(Debug, Default, Copy, Clone)]
pub(super) struct BiquadFilter {
    pub(super) b0: f32,
    pub(super) b1: f32,
    pub(super) b2: f32,
    pub(super) a1: f32,
    pub(super) a2: f32,
}

impl BiquadFilter {
    /// Applies the biquad transform to the given sample using the given coefficients and previous states.
    ///
    /// # Arguments
    ///
    /// * `sample` - The current input sample.
    /// * `x1` - The previous input sample.
    /// * `x2` - The sample before the previous input sample.
    /// * `y1` - The previous output sample.
    /// * `y2` - The sample before the previous output sample.
    ///
    /// # Returns
    ///
    /// The output sample after applying the biquad transform.
    pub(super) fn biquad_transform(
        &self,
        sample: f32,
        x1: f32,
        x2: f32,
        y1: f32,
        y2: f32,
    ) -> f32 {
        let BiquadFilter { b0, b1, b2, a1, a2 } = self;
        b0 * sample + b1 * x1 + b2 * x2 - a1 * y1 - a2 * y2
    }
}