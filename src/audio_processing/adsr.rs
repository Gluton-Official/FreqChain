use std::f32::EPSILON;
use nih_plug::prelude::*;
use realfft::num_traits::{Float, Inv};
use crate::audio_processing::util::BufferUtils;
use crate::freqchain;

// TODO: rename to smoothing-something
#[derive(Params)]
pub struct ADSRParams {
    #[id = "attack_bypass"]
    pub attack_bypass: BoolParam,
    #[id = "attack_speed"]
    pub attack_speed: FloatParam,

    #[id = "decay_bypass"]
    pub decay_bypass: BoolParam,
    #[id = "decay_speed"]
    pub decay_speed: FloatParam,
}

// TODO: change CHANNELS to
pub struct ADSR<const CHANNELS: usize> {
    current_amplitude: [f32; CHANNELS],

    was_attack_bypassed: [bool; CHANNELS],
}

pub enum ADSRState {
    ATTACK,
    DECAY,
}

impl<const CHANNELS: usize> Default for ADSR<CHANNELS> {
    fn default() -> Self {
        Self {
            current_amplitude: [0.0; CHANNELS],

            was_attack_bypassed: [true; CHANNELS],
        }
    }
}

impl<const CHANNELS: usize> ADSR<CHANNELS> {
    // TODO: investigate where debug parameters could be added
    pub fn process(&mut self, buffer: &mut Buffer, sample_rate: f32, params: &ADSRParams) {
        // TODO: average amplitude of multiple samples
        buffer.on_each_sample(move |channel, sample_index, sample| {
            let sample_amplitude = sample.abs();

            // TODO: this has to be moved out of the loop for when using with frequency bins
            if self.was_attack_bypassed[channel] && !params.attack_bypass.value() {
                // TODO: apply weighting
                self.current_amplitude[channel] = sample_amplitude / 3.0 + self.current_amplitude[channel] * 2.0 / 3.0;
            }
            self.was_attack_bypassed[channel] = params.attack_bypass.value();

            if !params.attack_bypass.value() && sample_amplitude > self.current_amplitude[channel] {
                // Attack
                let attack_speed = params.attack_speed.value();
                if attack_speed != 1f32 {
                    self.current_amplitude[channel] = exp_decay(self.current_amplitude[channel], sample_amplitude, attack_speed, sample_rate);
                    *sample *= self.current_amplitude[channel] / sample_amplitude;
                } else {
                    // TODO: add a self.was_attack_one flag if need more performance
                    self.current_amplitude[channel] = sample_amplitude;
                }
            } else if !params.decay_bypass.value() && sample_amplitude < self.current_amplitude[channel] {
                // Decay
                let decay_speed = params.decay_speed.value();
                if decay_speed != 1f32 {
                    self.current_amplitude[channel] = exp_decay(self.current_amplitude[channel], sample_amplitude, decay_speed, sample_rate);
                    *sample *= self.current_amplitude[channel] / sample_amplitude;
                } else {
                    // TODO: add a self.was_decay_one flag if need more performance
                    self.current_amplitude[channel] = sample_amplitude;
                }
            }
        });
    }
}

fn exp_decay(current: f32, target: f32, decay: f32, sample_rate: f32) -> f32 {
    target + (current - target) * (-decay * sample_rate.inv()).exp()
    // if (smoothed - target).abs() < 1e-5 {
    //     target
    // } else {
    //     smoothed
    // }
}

impl Default for ADSRParams {
    fn default() -> Self {
        Self {
            attack_bypass: BoolParam::new("Attack Bypass", true),
            attack_speed: FloatParam::new(
                "Attack Speed",
                1.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 1.0,
                    factor: FloatRange::skew_factor(1.0),
                }
            )
                .with_unit("%")
                .with_value_to_string(formatters::v2s_f32_percentage(0))
                .with_string_to_value(formatters::s2v_f32_percentage()),

            decay_bypass: BoolParam::new("Decay Bypass", true),
            decay_speed: FloatParam::new(
                "Decay Speed",
                1.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 1.0,
                    factor: FloatRange::skew_factor(1.0),
                }
            )
                .with_unit("%")
                .with_value_to_string(formatters::v2s_f32_percentage(0))
                .with_string_to_value(formatters::s2v_f32_percentage()),
        }
    }
}