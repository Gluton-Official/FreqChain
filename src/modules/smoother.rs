use std::ops::Not;

use nih_plug::prelude::*;

#[derive(Params)]
pub struct SmootherParams {
    #[id = "attack_bypass"]
    pub attack_bypass: BoolParam,
    #[id = "attack_speed"]
    pub attack_speed: FloatParam,

    #[id = "decay_bypass"]
    pub decay_bypass: BoolParam,
    #[id = "decay_speed"]
    pub decay_speed: FloatParam,
}

#[derive(Default, Clone)]
pub struct Smoother {
    current_value: Option<f32>,
}

impl Smoother {
    pub fn process(&mut self, value: &mut f32, sample_rate: f32, params: &SmootherParams) {
        if params.attack_bypass.value() && params.decay_bypass.value() {
            self.current_value = None;
            return;
        }

        if self.current_value.is_none() {
            self.current_value = Some(*value);
            return;
        }

        let current_value = self.current_value.unwrap();

        let speed = if *value > current_value {
            params.attack_bypass.value().not().then_some(params.attack_speed.value())
        } else if *value < current_value {
            params.decay_bypass.value().not().then_some(params.decay_speed.value())
        } else {
            return;
        };

        if let Some(speed) = speed.filter(|&speed| speed < 1.0) {
            *value = weighted_average(current_value, *value, sample_rate.powf(1.0 - speed.clamp(0.0, 1.0)));
        }
        self.current_value = Some(*value);
    }
}

fn weighted_average(current: f32, target: f32, weight: f32) -> f32 {
    (current * (weight - 1.0) + target) / weight
}

impl Default for SmootherParams {
    fn default() -> Self {
        Self {
            attack_bypass: BoolParam::new("Attack Bypass", true),
            attack_speed: FloatParam::new(
                "Attack Speed",
                1.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 1.0,
                    factor: FloatRange::skew_factor(0.5),
                },
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
                    factor: FloatRange::skew_factor(0.5),
                },
            )
            .with_unit("%")
            .with_value_to_string(formatters::v2s_f32_percentage(0))
            .with_string_to_value(formatters::s2v_f32_percentage()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_RATE: f32 = 44100.0;

    fn create_params(attack_bypass: bool, attack_speed: f32, decay_bypass: bool, decay_speed: f32) -> SmootherParams {
        SmootherParams {
            attack_bypass: BoolParam::new("Attack Bypass", attack_bypass),
            attack_speed: FloatParam::new("Attack Speed", attack_speed, FloatRange::Linear { min: 0.0, max: 1.0 }),
            decay_bypass: BoolParam::new("Decay Bypass", decay_bypass),
            decay_speed: FloatParam::new("Decay Speed", decay_speed, FloatRange::Linear { min: 0.0, max: 1.0 }),
        }
    }

    #[test]
    fn test_bypassed_attack_and_decay() {
        let mut smoother = Smoother::default();
        let params = create_params(true, 1.0, true, 1.0);
        let mut value: f32 = 0.5;

        smoother.process(&mut value, SAMPLE_RATE, &params);

        assert_eq!(value, 0.5);
        // Check internal state
        assert_eq!(smoother.current_value, None);
    }

    #[test]
    fn test_bypassed_attack_and_decay_with_non_one_speeds() {
        let mut smoother = Smoother::default();
        let params = create_params(true, 0.5, true, 0.5);
        let mut value: f32 = 0.5;

        smoother.process(&mut value, SAMPLE_RATE, &params);

        assert_eq!(value, 0.5);
        // Check internal state
        assert_eq!(smoother.current_value, None);
    }

    mod attack {
        use super::*;

        #[test]
        fn test_attack_with_speed_one_from_initial_state() {
            let mut smoother = Smoother::default();
            let params = create_params(false, 1.0, true, 1.0);
            let mut value: f32 = 0.5;

            smoother.process(&mut value, SAMPLE_RATE, &params);

            assert_eq!(value, 0.5);
            // Check internal state
            assert_eq!(smoother.current_value, Some(0.5));
        }

        #[test]
        fn test_attack_with_speed_one_from_same_value() {
            let mut smoother = Smoother::default();
            let params = create_params(false, 1.0, true, 1.0);
            let mut value: f32 = 0.5;

            // Force internal state
            smoother.current_value = Some(0.5);

            smoother.process(&mut value, SAMPLE_RATE, &params);

            assert_eq!(value, 0.5);
            // Check internal state
            assert_eq!(smoother.current_value, Some(0.5));
        }

        #[test]
        fn test_attack_with_speed_one_from_lower_value() {
            let mut smoother = Smoother::default();
            let params = create_params(false, 1.0, true, 1.0);
            let mut value: f32 = 0.5;

            // Force internal state
            smoother.current_value = Some(0.1);

            smoother.process(&mut value, SAMPLE_RATE, &params);

            assert_eq!(value, 0.5);
            // Check internal state
            assert_eq!(smoother.current_value, Some(0.5));
        }

        #[test]
        fn test_attack_with_non_one_speed_from_initial_state() {
            let mut smoother = Smoother::default();
            let params = create_params(false, 0.5, true, 1.0);
            let mut value: f32 = 0.5;

            smoother.process(&mut value, SAMPLE_RATE, &params);

            assert_eq!(value, 0.5);
            // Check internal state
            assert_eq!(smoother.current_value, Some(0.5));
        }

        #[test]
        fn test_attack_with_non_one_speed_from_same_value() {
            let mut smoother = Smoother::default();
            let params = create_params(false, 0.5, true, 1.0);
            let mut value: f32 = 0.5;

            // Force internal state
            smoother.current_value = Some(0.5);

            smoother.process(&mut value, SAMPLE_RATE, &params);

            assert_eq!(value, 0.5);
            // Check internal state
            assert_eq!(smoother.current_value, Some(0.5));
        }

        #[test]
        fn test_attack_with_non_one_speed_from_lower_value() {
            let mut smoother = Smoother::default();
            let params = create_params(false, 0.5, true, 1.0);
            let mut value: f32 = 0.5;

            // Force internal state
            smoother.current_value = Some(0.1);

            smoother.process(&mut value, SAMPLE_RATE, &params);

            assert!(value > 0.1 && value < 0.5);
            // Check internal state
            assert!(smoother.current_value > Some(0.1) && smoother.current_value < Some(0.5));
        }

        #[test]
        fn test_bypassed_decay_with_attack_speed_one_from_higher_value() {
            let mut smoother = Smoother::default();
            let params = create_params(false, 1.0, true, 1.0);
            let mut value: f32 = 0.5;

            // Force internal state
            smoother.current_value = Some(0.9);

            smoother.process(&mut value, SAMPLE_RATE, &params);

            assert_eq!(value, 0.5);
            // Check internal state
            assert_eq!(smoother.current_value, Some(0.5));
        }

        #[test]
        fn test_bypassed_decay_with_non_one_attack_speed_from_higher_value() {
            let mut smoother = Smoother::default();
            let params = create_params(false, 0.5, true, 1.0);
            let mut value: f32 = 0.5;

            // Force internal state
            smoother.current_value = Some(0.9);

            smoother.process(&mut value, SAMPLE_RATE, &params);

            assert_eq!(value, 0.5);
            // Check internal state
            assert_eq!(smoother.current_value, Some(0.5));
        }
    }

    mod decay {
        use super::*;

        #[test]
        fn test_decay_with_speed_one_from_initial_state() {
            let mut smoother = Smoother::default();
            let params = create_params(true, 1.0, false, 1.0);
            let mut value: f32 = 0.5;

            smoother.process(&mut value, SAMPLE_RATE, &params);

            assert_eq!(value, 0.5);
            // Check internal state
            assert_eq!(smoother.current_value, Some(0.5));
        }

        #[test]
        fn test_decay_with_speed_one_from_same_value() {
            let mut smoother = Smoother::default();
            let mut value: f32 = 0.5;
            let params = create_params(true, 1.0, false, 1.0);

            // Force initial value
            smoother.current_value = Some(0.5);

            smoother.process(&mut value, SAMPLE_RATE, &params);

            assert_eq!(value, 0.5);
            // Check internal state
            assert_eq!(smoother.current_value, Some(0.5));
        }

        #[test]
        fn test_decay_with_speed_one_from_higher_value() {
            let mut smoother = Smoother::default();
            let params = create_params(true, 1.0, false, 1.0);
            let mut value: f32 = 0.5;

            // Force internal state
            smoother.current_value = Some(0.9);

            smoother.process(&mut value, SAMPLE_RATE, &params);

            assert_eq!(value, 0.5);
            // Check internal state
            assert_eq!(smoother.current_value, Some(0.5));
        }

        #[test]
        fn test_decay_with_non_one_speed_from_initial_state() {
            let mut smoother = Smoother::default();
            let params = create_params(true, 1.0, false, 0.5);
            let mut value: f32 = 0.5;

            smoother.process(&mut value, SAMPLE_RATE, &params);

            assert_eq!(value, 0.5);
            // Check internal state
            assert_eq!(smoother.current_value, Some(0.5));
        }

        #[test]
        fn test_decay_with_non_one_speed_from_same_value() {
            let mut smoother = Smoother::default();
            let params = create_params(true, 1.0, false, 0.5);
            let mut value: f32 = 0.5;

            // Force internal state
            smoother.current_value = Some(0.5);

            smoother.process(&mut value, SAMPLE_RATE, &params);

            assert_eq!(value, 0.5);
            // Check internal state
            assert_eq!(smoother.current_value, Some(0.5));
        }

        #[test]
        fn test_decay_with_non_one_speed_from_higher_value() {
            let mut smoother = Smoother::default();
            let params = create_params(true, 1.0, false, 0.5);
            let mut value: f32 = 0.5;

            // Force internal state
            smoother.current_value = Some(0.9);

            smoother.process(&mut value, SAMPLE_RATE, &params);

            assert!(value > 0.5 && value < 0.9);
            // Check internal state
            assert!(smoother.current_value > Some(0.5) && smoother.current_value < Some(0.9));
        }

        #[test]
        fn test_bypassed_attack_with_decay_speed_one_from_lower_value() {
            let mut smoother = Smoother::default();
            let params = create_params(true, 1.0, false, 1.0);
            let mut value: f32 = 0.5;

            // Force internal state
            smoother.current_value = Some(0.1);

            smoother.process(&mut value, SAMPLE_RATE, &params);

            assert_eq!(value, 0.5);
            // Check internal state
            assert_eq!(smoother.current_value, Some(0.5));
        }

        #[test]
        fn test_bypassed_attack_with_non_one_decay_speed_from_lower_value() {
            let mut smoother = Smoother::default();
            let params = create_params(true, 1.0, false, 0.5);
            let mut value: f32 = 0.5;

            // Force internal state
            smoother.current_value = Some(0.1);

            smoother.process(&mut value, SAMPLE_RATE, &params);

            assert_eq!(value, 0.5);
            // Check internal state
            assert_eq!(smoother.current_value, Some(0.5));
        }
    }

    #[test]
    fn test_attack_and_decay_with_speed_one_from_initial_state() {
        let mut smoother = Smoother::default();
        let params = create_params(false, 1.0, false, 1.0);
        let mut value: f32 = 0.5;

        smoother.process(&mut value, SAMPLE_RATE, &params);

        assert_eq!(value, 0.5);
        // Check internal state
        assert_eq!(smoother.current_value, Some(0.5));
    }

    #[test]
    fn test_attack_and_decay_with_speed_one_from_same_value() {
        let mut smoother = Smoother::default();
        let params = create_params(false, 1.0, false, 1.0);
        let mut value: f32 = 0.5;

        // Force internal state
        smoother.current_value = Some(0.5);

        smoother.process(&mut value, SAMPLE_RATE, &params);

        assert_eq!(value, 0.5);
        // Check internal state
        assert_eq!(smoother.current_value, Some(0.5));
    }

    #[test]
    fn test_attack_and_decay_with_speed_one_from_lower_value() {
        let mut smoother = Smoother::default();
        let params = create_params(false, 1.0, false, 1.0);
        let mut value: f32 = 0.5;

        // Force internal state
        smoother.current_value = Some(0.1);

        smoother.process(&mut value, SAMPLE_RATE, &params);

        assert_eq!(value, 0.5);
        // Check internal state
        assert_eq!(smoother.current_value, Some(0.5));
    }

    #[test]
    fn test_attack_and_decay_with_speed_one_from_higher_value() {
        let mut smoother = Smoother::default();
        let params = create_params(false, 1.0, false, 1.0);
        let mut value: f32 = 0.5;

        // Force internal state
        smoother.current_value = Some(0.9);

        smoother.process(&mut value, SAMPLE_RATE, &params);

        assert_eq!(value, 0.5);
        // Check internal state
        assert_eq!(smoother.current_value, Some(0.5));
    }

    #[test]
    fn test_non_one_attack_speed_with_decay_with_speed_one_from_higher_value() {
        let mut smoother = Smoother::default();
        let params = create_params(false, 0.5, false, 1.0);
        let mut value: f32 = 0.5;

        // Force internal state
        smoother.current_value = Some(0.9);

        smoother.process(&mut value, SAMPLE_RATE, &params);

        assert_eq!(value, 0.5);
        // Check internal state
        assert_eq!(smoother.current_value, Some(0.5));
    }

    #[test]
    fn test_non_one_attack_speed_with_decay_with_speed_one_from_lower_value() {
        let mut smoother = Smoother::default();
        let params = create_params(false, 0.5, false, 1.0);
        let mut value: f32 = 0.5;

        // Force internal state
        smoother.current_value = Some(0.1);

        smoother.process(&mut value, SAMPLE_RATE, &params);

        assert!(value > 0.1 && value < 0.5);
        // Check internal state
        assert!(smoother.current_value > Some(0.1) && smoother.current_value < Some(0.5));
    }

    #[test]
    fn test_non_one_decay_speed_with_attack_with_speed_one_from_higher_value() {
        let mut smoother = Smoother::default();
        let params = create_params(false, 1.0, false, 0.5);
        let mut value: f32 = 0.5;

        // Force internal state
        smoother.current_value = Some(0.9);

        smoother.process(&mut value, SAMPLE_RATE, &params);

        assert!(value > 0.5 && value < 0.9);
        // Check internal state
        assert!(smoother.current_value > Some(0.5) && smoother.current_value < Some(0.9));
    }

    mod multiple_values {
        use super::*;

        #[test]
        fn test_attack_with_instant_decay_from_initial_state() {
            let mut smoother = Smoother::default();
            let params = create_params(false, 0.5, false, 1.0);
            let values= &mut [0.5, 0.9, 0.1];

            for mut value in values.iter_mut() {
                smoother.process(&mut value, SAMPLE_RATE, &params);
            }

            assert_eq!(*values.last().unwrap(), 0.1);
            // Check internal state
            assert_eq!(smoother.current_value, Some(0.1));
        }

        #[test]
        fn test_attack_with_instant_decay_from_same_value() {
            let mut smoother = Smoother::default();
            let params = create_params(false, 0.5, false, 1.0);
            let values= &mut [0.5, 0.9, 0.1];

            // Force internal state
            smoother.current_value = Some(0.5);

            for mut value in values.iter_mut() {
                smoother.process(&mut value, SAMPLE_RATE, &params);
            }

            assert_eq!(*values.last().unwrap(), 0.1);
            // Check internal state
            assert_eq!(smoother.current_value, Some(0.1));
        }

        #[test]
        fn test_slow_attack_and_fast_decay_from_same_value() {
            let mut smoother = Smoother::default();
            let params = create_params(false, 0.1, false, 0.9);
            let values= &mut [0.5, 0.6, 0.1];

            // Force internal state
            smoother.current_value = Some(0.5);

            for mut value in values.iter_mut() {
                smoother.process(&mut value, SAMPLE_RATE, &params);
            }

            let last= *values.last().unwrap();
            assert!(last > 0.1 && last < 0.5);
            // Check internal state
            assert!(smoother.current_value > Some(0.1) && smoother.current_value < Some(0.5));
        }
    }
}
