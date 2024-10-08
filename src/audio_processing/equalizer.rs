use std::f32::consts::TAU;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use nih_plug::prelude::*;

use super::biquad_filter::BiquadFilter;

pub struct Equalizer<const N: usize> {
    sample_rate: f32,
    biquad_filters: [BiquadFilter; N],
}

#[derive(Params)]
pub struct EqualizerParams<const N: usize> {
    #[nested(array, group = "Band")]
    pub bands: [BandParams; N],

    #[id = "bypass"]
    pub bypass: BoolParam,
}

#[derive(Params)]
pub struct BandParams {
    #[id = "band_type"]
    pub band_type: EnumParam<BandType>,
    #[id = "frequency"]
    pub frequency: FloatParam,
    #[id = "q"]
    pub q: FloatParam,
    #[id = "gain"]
    pub gain: FloatParam,

    dirty: Arc<AtomicBool>,
}

#[derive(Enum, Debug, PartialEq)]
pub enum BandType {
    #[id = "peak"]
    Peak,
    #[id = "notch"]
    Notch,
    #[id = "high_shelf"]
    #[name = "High Shelf"]
    HighShelf,
    #[id = "low_shelf"]
    #[name = "Low Shelf"]
    LowShelf,
    #[id = "high_pass"]
    #[name = "High Pass"]
    HighPass,
    #[id = "low_pass"]
    #[name = "Low Pass"]
    LowPass,
}

impl<const N: usize> Default for Equalizer<N> {
    fn default() -> Self {
        Self {
            sample_rate: 1.0,
            biquad_filters: [Default::default(); N],
        }
    }
}

impl<const N: usize> Equalizer<N> {
    pub fn process(&mut self, buffer: &mut Buffer, params: &EqualizerParams<N>) {
        if params.bypass.value() {
            return;
        }

        nih_debug_assert_ne!(self.sample_rate, 1_f32);

        for (band_params, band_filters) in params.bands.iter().zip(self.biquad_filters.iter_mut()) {
            // skip processing for peak and shelf filters when gain is 0db
            if matches!(
                band_params.band_type.value(),
                BandType::Peak | BandType::HighShelf | BandType::LowShelf
            ) && band_params.gain.value() == 0_f32 {
                continue;
            }

            // update coefficients if dirty (i.e. the parameters were changed)
            if band_params.dirty.load(Ordering::SeqCst) {
                *band_filters = band_params.calculate_filter(self.sample_rate);
                band_params.dirty.store(false, Ordering::SeqCst);
            }

            let mut x = [[0_f32; 2]; 2]; // input sample history, newest to oldest
            let mut y = [[0_f32; 2]; 2]; // output sample history, newest to oldest
            for channel_samples in buffer.iter_samples() {
                for (channel_index, sample) in channel_samples.into_iter().enumerate() {
                    let result = band_filters.biquad_transform(
                        *sample,
                        x[channel_index][0],
                        x[channel_index][1],
                        y[channel_index][0],
                        y[channel_index][1],
                    );
                    x[channel_index][1] = x[channel_index][0];
                    x[channel_index][0] = *sample;
                    y[channel_index][1] = y[channel_index][0];
                    y[channel_index][0] = result;

                    *sample = result;
                }
            }
        }
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }
}

impl BandParams {
    fn new(band_number: i32, band_type: BandType, frequency: f32, q: f32, gain: f32) -> Self {
        let dirty = Arc::new(AtomicBool::new(false));

        let band_name = format!("EQ Band {band_number}");
        Self {
            band_type: EnumParam::new(format!("{band_name} Type"), band_type)
                .with_callback(Arc::new({
                    let dirty = dirty.clone();
                    move |_| dirty.store(true, Ordering::SeqCst)
                })),
            frequency: FloatParam::new(
                format!("{band_name} Frequency"),
                frequency,
                FloatRange::Skewed { min: 20.0, max: 20_000.0, factor: FloatRange::skew_factor(-2.0) }
            )
                .with_value_to_string(formatters::v2s_f32_hz_then_khz(0))
                .with_string_to_value(formatters::s2v_f32_hz_then_khz())
                .with_callback(Arc::new({
                    let dirty = dirty.clone();
                    move |_| dirty.store(true, Ordering::SeqCst)
                })),
            q: FloatParam::new(
                format!("{band_name} Q"),
                q,
                FloatRange::Skewed { min: 0.1, max: 18.0, factor: FloatRange::skew_factor(-2.0) }
            )
                .with_step_size(0.01)
                .with_callback(Arc::new({
                    let dirty = dirty.clone();
                    move |_| dirty.store(true, Ordering::SeqCst)
                })),
            gain: FloatParam::new(
                format!("{band_name} Gain"),
                gain,
                FloatRange::Linear { min: -18.0, max: 18.0 },
            )
                .with_unit("dB")
                .with_value_to_string(formatters::v2s_f32_rounded(1))
                .with_string_to_value(Arc::new(move |string| string.trim_end_matches("dB").parse().ok()))
                .with_callback(Arc::new({
                    let dirty = dirty.clone();
                    move |_| dirty.store(true, Ordering::SeqCst)
                })),

            dirty,
        }
    }

    #[allow(non_snake_case)]
    fn calculate_filter(&self, sample_rate: f32) -> BiquadFilter {
        let w0 = TAU * self.frequency.value() / sample_rate;
        let sin_w0 = w0.sin();
        let cos_w0 = w0.cos();
        match self.band_type.value() {
            BandType::Peak => {
                let A = 10_f32.powf(self.gain.value() / 40_f32);
                let alpha = sin_w0 * (2_f32.ln() / 2_f32 * self.bandwidth(w0) * w0 / sin_w0).sinh();
                let a0 = 1_f32 + alpha / A;
                BiquadFilter {
                    b0:  (1_f32 + alpha * A) / a0,
                    b1: (-2_f32 * cos_w0)    / a0,
                    b2:  (1_f32 - alpha * A) / a0,
                    a1: (-2_f32 * cos_w0)    / a0,
                    a2:  (1_f32 - alpha / A) / a0,
                }
            }
            BandType::Notch => {
                let alpha = sin_w0 * (2_f32.ln() / 2_f32 * self.bandwidth(w0) * w0 / sin_w0).sinh();
                let a0 = 1_f32 + alpha;
                BiquadFilter {
                    b0:   1_f32           / a0,
                    b1: (-2_f32 * cos_w0) / a0,
                    b2:   1_f32           / a0,
                    a1: (-2_f32 * cos_w0) / a0,
                    a2:  (1_f32 - alpha)  / a0,
                }
            }
            BandType::HighShelf => {
                let A = 10_f32.powf(self.gain.value() / 40_f32);
                let alpha = sin_w0 / 2_f32 * ((A + 1_f32 / A) * (1_f32 / self.shelf_slope(A) - 1_f32)).sqrt();
                let sqrt_A_alpha_2 = 2_f32 * A.sqrt() * alpha;
                let a0 = (A + 1_f32) - (A - 1_f32) * cos_w0 + sqrt_A_alpha_2;
                BiquadFilter {
                    b0:          (A * ((A + 1_f32) + (A - 1_f32) * cos_w0 + sqrt_A_alpha_2)) / a0,
                    b1: (-2_f32 * A * ((A - 1_f32) + (A + 1_f32) * cos_w0))                  / a0,
                    b2:          (A * ((A + 1_f32) + (A - 1_f32) * cos_w0 - sqrt_A_alpha_2)) / a0,
                    a1:      (2_f32 * ((A - 1_f32) - (A + 1_f32) * cos_w0))                  / a0,
                    a2:               ((A + 1_f32) - (A - 1_f32) * cos_w0 - sqrt_A_alpha_2)  / a0,
                }
            }
            BandType::LowShelf => {
                let A = 10_f32.powf(self.gain.value() / 40_f32);
                let alpha = sin_w0 / 2_f32 * ((A + 1_f32 / A) * (1_f32 / self.shelf_slope(A) - 1_f32)).sqrt();
                let sqrt_A_alpha_2 = 2_f32 * A.sqrt() * alpha;
                let a0 = (A + 1_f32) + (A - 1_f32) * cos_w0 + sqrt_A_alpha_2;
                BiquadFilter {
                    b0:         (A * ((A + 1_f32) - (A - 1_f32) * cos_w0 + sqrt_A_alpha_2)) / a0,
                    b1: (2_f32 * A * ((A - 1_f32) - (A + 1_f32) * cos_w0))                  / a0,
                    b2:         (A * ((A + 1_f32) - (A - 1_f32) * cos_w0 - sqrt_A_alpha_2)) / a0,
                    a1:    (-2_f32 * ((A - 1_f32) + (A + 1_f32) * cos_w0))                  / a0,
                    a2:              ((A + 1_f32) + (A - 1_f32) * cos_w0 - sqrt_A_alpha_2)  / a0,
                }
            }
            BandType::LowPass => {
                let alpha = sin_w0 / (2_f32 * self.resonance());
                let a0 = 1_f32 + alpha;
                BiquadFilter {
                    b0: ((1_f32 - cos_w0) / 2_f32) / a0,
                    b1:  (1_f32 - cos_w0)          / a0,
                    b2: ((1_f32 - cos_w0) / 2_f32) / a0,
                    a1: (-2_f32 * cos_w0)          / a0,
                    a2:  (1_f32 - alpha)           / a0,
                }
            }
            BandType::HighPass => {
                let alpha = sin_w0 / (2_f32 * self.resonance());
                let a0 = 1_f32 + alpha;
                BiquadFilter {
                    b0:  ((1_f32 + cos_w0) / 2_f32) / a0,
                    b1: (-(1_f32 + cos_w0))         / a0,
                    b2:  ((1_f32 + cos_w0) / 2_f32) / a0,
                    a1:  (-2_f32 * cos_w0)          / a0,
                    a2:   (1_f32 - alpha)           / a0,
                }
            }
        }
    }

    fn resonance(&self) -> f32 {
        self.q.value()
    }

    fn bandwidth(&self, w0: f32) -> f32 {
        ((1_f32 / self.q.value().clamp(f32::MIN, 1.0)) / 2_f32).asinh() / (2_f32.ln() / 2_f32 * w0 / w0.sin())
    }

    #[allow(non_snake_case)]
    fn shelf_slope(&self, A: f32) -> f32 {
        1_f32 / (((1_f32 / self.q.value().clamp(f32::MIN, 1.0)).powi(2) - 2_f32) / A + 1_f32 / A + 1_f32)
    }
}


impl Default for EqualizerParams<7> {
    fn default() -> Self {
        Self {
            bands: [
                BandParams::new(1, BandType::LowShelf, 63.0, 1.0, 0.0),
                BandParams::new(2, BandType::Peak, 136.0, 1.0, 0.0),
                BandParams::new(3, BandType::Peak, 294.0, 1.0, 0.0),
                BandParams::new(4, BandType::Peak, 632.0, 1.0, 0.0),
                BandParams::new(5, BandType::Peak, 1363.0, 1.0, 0.0),
                BandParams::new(6, BandType::Peak, 2936.0, 1.0, 0.0),
                BandParams::new(7, BandType::HighShelf, 6324.0, 1.0, 0.0),
            ],
            bypass: BoolParam::new("Bypass", false),
        }
    }
}
