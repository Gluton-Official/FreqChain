use std::ops::Sub;
use nih_plug::prelude::FloatRange;
use nih_plug::util;
use nih_plug_iced::{assets, rule, text_input, Background, Button, Color, Font, Point, Size, Vector};
use nih_plug_iced::canvas::{Path, Stroke};
use crate::ui::ColorUtils;
use crate::ui::widgets::{param_knob, param_slider, param_toggle};
use crate::ui::widgets::param_knob::{KnobStyle, PointerStyle};
use crate::ui::widgets::param_toggle::ButtonStyle;
use crate::ui::widgets::value_input::TextInputStyle;

#[derive(Clone, Copy)]
pub struct Theme {
    pub foreground: Color,
    pub background: Color,

    pub font: Font,
    pub text_size: u16,
}

pub struct FreqChainTheme;

impl FreqChainTheme {
    pub fn dark() -> Theme {
        Theme {
            foreground: Color::from_rgb8(0xE4, 0xE4, 0xE4),
            background: Color::from_rgb8(0x14, 0x14, 0x14),
            
            font: assets::NOTO_SANS_LIGHT,
            text_size: 12,
        }
    }
}

impl rule::StyleSheet for Theme {
    fn style(&self) -> rule::Style {
        rule::Style {
            color: self.foreground.with_alpha(0.5),
            
            ..rule::Style::default()
        }
    }
}

impl text_input::StyleSheet for Theme {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            background: Background::Color(self.background.lerp_to_inverse(0.1)),
            border_radius: 0.0,
            border_width: 1.0,
            border_color: self.background.lerp_to_inverse(0.25),
        }
    }
    fn focused(&self) -> text_input::Style { self.active() }
    fn placeholder_color(&self) -> Color { self.foreground.with_alpha(0.5) }
    fn value_color(&self) -> Color { self.foreground }
    fn selection_color(&self) -> Color { self.foreground.with_alpha(0.5) }
}

impl param_knob::StyleSheet for Theme {
    fn style(&self) -> param_knob::Style {
        param_knob::Style {
            font: self.font,
            text_size: self.text_size,
            text_color: self.foreground,
            
            knob: Some(KnobStyle {
                stroke: Stroke::default().with_color(self.foreground),
                fill: Some(self.background.into()),
                
                ..KnobStyle::default()
            }),
            pointer: Some(PointerStyle {
                stroke: Stroke::default().with_color(self.foreground),
                
                ..PointerStyle::default()
            }),
            track: Some(param_knob::TrackStyle {
                filled_stroke: Stroke::default().with_color(self.foreground),
                unfilled_stroke: Stroke::default().with_color(self.foreground.with_alpha(0.25)),
                
                ..param_knob::TrackStyle::default()
            }),
            text_input: TextInputStyle {
                background: self.background.lerp_to_inverse(0.1).into(),
                border_color: self.background.lerp_to_inverse(0.25),
                value_color: self.foreground,
                placeholder_color: self.foreground.with_alpha(0.5),
                selection_color: self.foreground.with_alpha(0.5),
                
                ..TextInputStyle::default()
            },
            
            ..param_knob::Style::default()
        }
    }
    
    fn hovered(&self) -> param_knob::Style {
        param_knob::Style {
            knob: Some(KnobStyle {
                fill: Some(self.background.lerp_to(self.foreground, 0.25).into()),
                
                ..self.style().knob.unwrap_or_default()
            }),
            
            ..self.style()
        }
    }
}

impl param_slider::StyleSheet for Theme {
    fn style(&self) -> param_slider::Style {
        param_slider::Style {
            font: self.font,
            text_size: self.text_size,
            text_color: self.foreground,
            text_padding: 7.0,

            handle: Some(param_slider::HandleStyle {
                stroke: Stroke::default().with_color(self.foreground),
                fill: Some(self.background.into()),

                ..param_slider::HandleStyle::default()
            }),
            track: Some(param_slider::TrackStyle {
                filled_stroke: Stroke::default().with_color(self.foreground).with_width(4.0),
                unfilled_stroke: Stroke::default().with_color(self.foreground.with_alpha(0.25)),

                ..param_slider::TrackStyle::default()
            }),

            major_tick_marks: Some(param_slider::TickMarkStyle {
                values: vec![FloatRange::Skewed {
                    min: util::db_to_gain(util::MINUS_INFINITY_DB),
                    max: util::db_to_gain(24.0),
                    factor: FloatRange::gain_skew_factor(util::MINUS_INFINITY_DB, 24.0)
                }.normalize(util::db_to_gain(0.0))],
                stroke: Stroke::default().with_color(self.foreground),

                ..param_slider::TickMarkStyle::default()
            }),
            minor_tick_marks: Some(param_slider::TickMarkStyle {
                values: vec![0.0, 1.0],
                stroke: Stroke::default().with_color(self.foreground),
                draw_path: |orientation, center, scale| {
                    match orientation {
                        param_slider::Orientation::Vertical => Path::line([center.x - 0.25 * scale, center.y].into(), [center.x + 0.25 * scale, center.y].into()),
                        param_slider::Orientation::Horizontal => Path::line([center.x, center.y - 0.25 * scale].into(), [center.x, center.y + 0.25 * scale].into())
                    }
                },

                ..param_slider::TickMarkStyle::default()
            }),

            text_input: TextInputStyle {
                background: self.background.lerp_to_inverse(0.1).into(),
                border_color: self.background.lerp_to_inverse(0.25),
                value_color: self.foreground,
                placeholder_color: self.foreground.with_alpha(0.5),
                selection_color: self.foreground.with_alpha(0.5),

                ..TextInputStyle::default()
            },

            ..param_slider::Style::default()
        }
    }

    fn hovered(&self) -> param_slider::Style {
        param_slider::Style {
            handle: Some(param_slider::HandleStyle {
                fill: Some(self.background.lerp_to(self.foreground, 0.25).into()),

                ..self.style().handle.unwrap_or_default()
            }),

            ..self.style()
        }
    }
}

impl param_toggle::StyleSheet for Theme {
    fn style(&self) -> param_toggle::Style {
        param_toggle::Style {
            font: self.font,
            text_size: self.text_size,
            text_color: self.foreground,
            text_padding: 7.0,
            
            button: Some(ButtonStyle {
                stroke: Stroke::default().with_color(Color::TRANSPARENT),
                fill: Some(self.background.lerp_to_inverse(0.10).into()),
                
                ..ButtonStyle::default()
            }),
            
            ..param_toggle::Style::default()
        }
    }
    
    fn hovered(&self) -> param_toggle::Style {
        param_toggle::Style {
            button: Some(ButtonStyle {
                fill: Some(self.background.lerp_to_inverse(0.25).into()),
                
                ..self.style().button.unwrap_or_default()
            }),
            
            ..self.style()
        }
    }
    
    fn active(&self) -> param_toggle::Style {
        param_toggle::Style {
            button: Some(ButtonStyle {
                fill: Some(self.background.lerp_to_inverse(0.5).into()),

                ..self.style().button.unwrap_or_default()
            }),

            ..self.style()
        }
    }
    
    fn disabled(&self) -> param_toggle::Style {
        param_toggle::Style {
            text_color: self.foreground.lerp_to_inverse(0.5),

            ..self.style()
        }
    }
}

impl Theme {
    pub fn mono_toggle<'a>(self, mono_mode: bool) -> impl Into<Box<dyn param_toggle::StyleSheet + 'a>> {
        fn circle_radius(scale: Vector) -> f32 {
            let min = f32::min(scale.x, scale.y);
            1.0 / 3.0 * min
        }

        struct MonoToggle {
            theme: Theme,
            mono_mode: bool,
        }
        impl param_toggle::StyleSheet for MonoToggle {
            fn style(&self) -> param_toggle::Style {
                param_toggle::Style {
                    label_placement: None,

                    button: Some(ButtonStyle {
                        draw_path: match self.mono_mode {
                            true => |_value, center, scale| {
                                Path::circle(center, circle_radius(scale))
                            },
                            false => |_value, center, scale| {
                                let radius = circle_radius(scale);
                                let offset = Vector::new(radius * 2.0 / 3.0, 0.0);
                                Path::new(|path| {
                                    path.circle(center - offset, radius);
                                    path.circle(center + offset, radius);
                                })
                            },
                        },
                        stroke: Stroke::default().with_color(self.theme.foreground.lerp_to_inverse(0.50)),
                        fill: None,
                    }),

                    ..<Theme as param_toggle::StyleSheet>::style(&self.theme)
                }
            }

            fn hovered(&self) -> param_toggle::Style {
                param_toggle::Style {
                    button: Some(ButtonStyle {
                        stroke: Stroke::default().with_color(self.theme.foreground.lerp_to_inverse(0.25)),

                        ..self.style().button.unwrap_or(<Theme as param_toggle::StyleSheet>::style(&self.theme).button.unwrap_or_default())
                    }),

                    ..self.style()
                }
            }

            fn active(&self) -> param_toggle::Style {
                param_toggle::Style {
                    button: Some(ButtonStyle {
                        stroke: Stroke::default().with_color(self.theme.foreground),

                        ..self.style().button.unwrap_or(<Theme as param_toggle::StyleSheet>::style(&self.theme).button.unwrap_or_default())
                    }),

                    ..self.style()
                }
            }
        }
        MonoToggle {
            theme: self,
            mono_mode
        }
    }
}