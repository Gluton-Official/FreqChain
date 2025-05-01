use crate::util::biquad_filter::BiquadFilter;
use crate::util::remap::normalize;
use nih_plug::prelude::*;
use std::f32::consts::TAU;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;

pub struct Equalizer<const BANDS: usize, const CHANNELS: usize> {
    sample_rate: Option<f32>,
    biquad_filters: [BiquadFilter; BANDS],

    x1: [f32; CHANNELS],
    x2: [f32; CHANNELS],
    y1: [f32; CHANNELS],
    y2: [f32; CHANNELS],
}

#[derive(Params)]
pub struct EqualizerParams<const BANDS: usize> {
    #[nested(array, group = "Band")]
    pub bands: [BandParams; BANDS],
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
    #[id = "bypass"]
    pub bypass: BoolParam,

    dirty: Arc<AtomicBool>,
}

#[derive(Enum, Debug, Copy, Clone, PartialEq)]
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

impl<const BANDS: usize, const CHANNELS: usize> Default for Equalizer<BANDS, CHANNELS> {
    fn default() -> Self {
        Self {
            sample_rate: None,
            biquad_filters: [BiquadFilter::default(); BANDS],

            x1: [0_f32; CHANNELS],
            x2: [0_f32; CHANNELS],
            y1: [0_f32; CHANNELS],
            y2: [0_f32; CHANNELS],
        }
    }
}

impl<const BANDS: usize, const CHANNELS: usize> Equalizer<BANDS, CHANNELS> {
    pub fn process(&mut self, buffer: &mut Buffer, params: &EqualizerParams<BANDS>) {
        if self.sample_rate.is_none() {
            return;
        }

        // TODO: probably want to apply all band filters before moving to the next sample
        for (band_params, band_filter) in params.bands.iter().zip(self.biquad_filters.iter_mut()) {
            if band_params.bypass.value() {
                continue;
            }
            
            // skip processing for peak and shelf filters when gain is 0db
            if matches!(
                band_params.band_type.value(),
                BandType::Peak | BandType::HighShelf | BandType::LowShelf
            ) && band_params.gain.value() == 0_f32 {
                continue;
            }

            // update coefficients if dirty (i.e. the parameters were changed)
            if band_params.dirty.load(Ordering::SeqCst) {
                *band_filter = band_params.calculate_filter(self.sample_rate.unwrap());
                band_params.dirty.store(false, Ordering::SeqCst);
            }

            for channel_samples in buffer.iter_samples() {
                for (channel_index, sample) in channel_samples.into_iter().enumerate() {
                    let result = band_filter.biquad_transform(
                        *sample,
                        self.x1[channel_index],
                        self.x2[channel_index],
                        self.y1[channel_index],
                        self.y2[channel_index],
                    );
                    self.x2[channel_index] = self.x1[channel_index];
                    self.x1[channel_index] = *sample;
                    self.y2[channel_index] = self.y1[channel_index];
                    self.y1[channel_index] = result;

                    *sample = result;
                }
            }
        }
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = Some(sample_rate);
    }
}

impl BandParams {
    fn new(band_number: i32, band_type: BandType, frequency: f32, q: f32, gain: f32) -> Self {
        // Initialize as dirty just in case default bands actually do something
        let dirty = Arc::new(AtomicBool::new(true));

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
                {
                    let min = 0.1;
                    let max = 18.0;
                    let center = 1.0;
                    let factor = 0.5_f32.log(normalize(center, min, max)); // make 1 the center
                    FloatRange::Skewed { min, max, factor }
                }
            )
                .with_value_to_string(formatters::v2s_f32_rounded(2))
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
                .with_string_to_value(Arc::new(move |string| string.trim_end_matches("dB").trim_end().parse().ok()))
                .with_callback(Arc::new({
                    let dirty = dirty.clone();
                    move |_| dirty.store(true, Ordering::SeqCst)
                })),
            // isn't used for filter coefficients, so doesn't need callback
            bypass: BoolParam::new(format!("{band_name} Bypass"), false),

            dirty,
        }
    }

    #[allow(non_snake_case)]
    pub(crate) fn calculate_filter(&self, sample_rate: f32) -> BiquadFilter {
        // angular frequency
        let w0 = TAU * self.frequency.value() / sample_rate;
        let sin_w0 = w0.sin();
        let cos_w0 = w0.cos();
        let alpha = sin_w0 / (2_f32 * self.q.value());
        match self.band_type.value() {
            BandType::Peak => {
                // linear gain
                let A = 10_f32.powf(self.gain.value() / 40_f32);
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

    // TODO: reimplement for UI
    // fn resonance(&self) -> f32 {
    //     self.q.value()
    // }
    // 
    // 
    // fn bandwidth(&self, w0: f32) -> f32 {
    //     // ((1_f32 / self.q.value()) / 2_f32).asinh() / (LN_2 / 2_f32 * w0 / w0.sin())
    //     (2_f32 * w0.sin() * (2_f32 * self.q.value()).recip().asinh()) / (LN_2 * w0)
    //     // (LN_2 / 2_f32 * (1_f32 / self.q.value())).sinh() * LN_2
    // }
    // 
    // #[allow(non_snake_case)]
    // fn shelf_slope(&self, A: f32) -> f32 {
    //     // 1_f32 / (((1_f32 / self.q.value().clamp(f32::MIN, 1.0)).powi(2) - 2_f32) / A + 1_f32 / A + 1_f32)
    //     (((1_f32 / self.q.value().powf(2.0)) - 2_f32) / (A + A.recip()) + 1_f32).recip()
    // }
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
        }
    }
}