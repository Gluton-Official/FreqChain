use std::f32::consts::{FRAC_PI_2, FRAC_PI_8};
use crate::modules::equalizer::BandType;
use crate::ui::widgets::param_knob::{KnobStyle, PointerStyle};
use crate::ui::widgets::param_toggle::ButtonStyle;
use crate::ui::widgets::spectrum::{Shape, WaveStyle};
use crate::ui::widgets::value_input::TextInputStyle;
use crate::ui::widgets::{equalizer, param_knob, param_slider, param_toggle, spectrum};
use crate::ui::ColorUtils;
use nih_plug::prelude::FloatRange;
use nih_plug::util;
use nih_plug_iced::canvas::{Fill, FillRule, Path, Stroke};
use nih_plug_iced::{assets, container, rule, text_input, Background, Color, Font, Point, Vector};
use nih_plug_iced::canvas::path::Arc;

#[derive(Clone, Copy)]
pub struct Theme {
    pub foreground: Color,
    pub background: Color,
    
    pub orange: Color,
    pub yellow: Color,
    pub green: Color,
    pub cyan: Color,
    pub blue: Color,
    pub purple: Color,
    pub magenta: Color,

    pub font: Font,
    pub text_size: u16,
}

pub struct FreqChainTheme;

impl FreqChainTheme {
    pub fn dark() -> Theme {
        Theme {
            foreground: Color::from_rgb8(0xE4, 0xE4, 0xE4),
            background: Color::from_rgb8(0x14, 0x14, 0x14),
            
            orange: Color::from_rgb8(255, 83, 0),
            yellow: Color::from_rgb8(255, 255, 0),
            green: Color::from_rgb8(114, 255, 33),
            cyan: Color::from_rgb8(0, 255, 255),
            blue: Color::from_rgb8(0, 155, 233),
            purple: Color::from_rgb8(136, 33, 255),
            magenta: Color::from_rgb8(225, 20, 153),
            
            font: assets::NOTO_SANS_LIGHT,
            text_size: 12,
        }
    }
}

impl container::StyleSheet for Theme {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(self.foreground),
            background: Some(self.background.into()),
            border_radius: 0.0,
            border_width: 1.0,
            border_color: self.foreground.with_alpha(0.5),
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
            track: None,
            // track: Some(param_knob::TrackStyle {
            //     filled_stroke: Stroke::default().with_color(self.foreground),
            //     unfilled_stroke: Stroke::default().with_color(self.foreground.with_alpha(0.25)),
            //     
            //     ..param_knob::TrackStyle::default()
            // }),
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
                draw_path: |orientation, center, scale| {
                    let size = match orientation {
                        param_slider::Orientation::Vertical => Vector::new(0.8, 0.25) * scale,
                        param_slider::Orientation::Horizontal => Vector::new( 0.25, 0.8) * scale,
                    };
                    Path::rectangle(
                        Point::new(center.x - size.x / 2.0, center.y - size.y / 2.0),
                        size.into(),
                    )
                },
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

impl Theme {
    pub fn slider<'a>(self, center: f32) -> impl Into<Box<dyn param_slider::StyleSheet + 'a>> {
        struct Slider {
            theme: Theme,
            center: f32,
        }
        impl param_slider::StyleSheet for Slider {
            fn style(&self) -> param_slider::Style {
                param_slider::Style {
                    major_tick_marks: Some(param_slider::TickMarkStyle {
                        values: vec![self.center],
                        stroke: Stroke::default().with_color(self.theme.foreground),

                        ..<Theme as param_slider::StyleSheet>::style(&self.theme).major_tick_marks.unwrap_or_default()
                    }),
                    
                    ..<Theme as param_slider::StyleSheet>::style(&self.theme)
                }
            }

            fn hovered(&self) -> param_slider::Style {
                param_slider::Style {
                    major_tick_marks: self.style().major_tick_marks,
                    
                    ..<Theme as param_slider::StyleSheet>::hovered(&self.theme)
                }
            }
        }
        Slider {
            theme: self,
            center,
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

                        ..self.style().button.unwrap_or(<Theme as param_toggle::StyleSheet>::hovered(&self.theme).button.unwrap_or_default())
                    }),

                    ..self.style()
                }
            }

            fn active(&self) -> param_toggle::Style {
                param_toggle::Style {
                    button: Some(ButtonStyle {
                        stroke: Stroke::default().with_color(self.theme.foreground),

                        ..self.style().button.unwrap_or(<Theme as param_toggle::StyleSheet>::active(&self.theme).button.unwrap_or_default())
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

    pub fn radio_toggle<'a>(self, color: Option<Color>) -> impl Into<Box<dyn param_toggle::StyleSheet + 'a>> {
        struct RadioToggle {
            theme: Theme,
            color: Option<Color>,
        }
        impl param_toggle::StyleSheet for RadioToggle {
            fn style(&self) -> param_toggle::Style {
                param_toggle::Style {
                    label_placement: None,

                    button: Some(ButtonStyle {
                        draw_path: |_value, center, scale| {
                            let radius = f32::min(scale.x, scale.y) / 2.0;
                            Path::new(|path| {
                                path.circle(center, radius);
                                path.circle(center, radius * 0.9);
                            })
                        },
                        stroke: Stroke::default().with_color(Color::TRANSPARENT).with_width(0.0),
                        fill: Some(Fill {
                            color: self.color.unwrap_or(self.theme.foreground),
                            rule: FillRule::EvenOdd,
                        }),
                    }),

                    ..<Theme as param_toggle::StyleSheet>::style(&self.theme)
                }
            }

            fn active(&self) -> param_toggle::Style {
                param_toggle::Style {
                    button: Some(ButtonStyle {
                        draw_path: |_value, center, scale| {
                            let radius = f32::min(scale.x, scale.y) / 2.0;
                            Path::new(|path| {
                                path.circle(center, radius);
                                path.circle(center, radius * 0.9);
                                path.circle(center, radius / 3.0);
                            })
                        },

                        ..self.style().button.unwrap_or(<Theme as param_toggle::StyleSheet>::active(&self.theme).button.unwrap_or_default())
                    }),

                    ..self.style()
                }
            }
        }
        RadioToggle {
            theme: self,
            color,
        }
    }

    pub fn band_shape_toggle<'a>(self, shape: BandType, color: Color) -> impl Into<Box<dyn param_toggle::StyleSheet + 'a>> {
        struct BandShapeToggle {
            theme: Theme,
            shape: BandType,
            color: Color,
        }
        impl param_toggle::StyleSheet for BandShapeToggle {
            fn style(&self) -> param_toggle::Style {
                param_toggle::Style {
                    label_placement: None,

                    button: Some(ButtonStyle {
                        draw_path: match self.shape {
                            BandType::Peak => |_value, center, scale| {
                                let top = Point::new(center.x, center.y - scale.y / 2.0);
                                let bottom = Point::new(center.x, center.y + scale.y / 2.0);
                                let left = Point::new(center.x - scale.x / 2.0, center.y);
                                let right = Point::new(center.x + scale.x / 2.0, center.y);
                                let top_anchor = Point::new(center.x, center.y - scale.y / 4.0);
                                let bottom_anchor = Point::new(center.x, center.y + scale.y / 4.0);
                                let left_anchor = Point::new(center.x - scale.x / 4.0, center.y);
                                let right_anchor = Point::new(center.x + scale.x / 4.0, center.y);
                                Path::new(|path| {
                                    path.move_to(top);
                                    path.bezier_curve_to(top_anchor, left_anchor, left);
                                    path.bezier_curve_to(left_anchor, bottom_anchor, bottom);
                                    path.bezier_curve_to(bottom_anchor, right_anchor, right);
                                    path.bezier_curve_to(right_anchor, top_anchor, top);
                                })
                            },
                            BandType::Notch => |_value, center, scale| {
                                let top_left = center - Vector::new(scale.x / 2.0, scale.y / 2.0);
                                let left_anchor = Point::new(center.x - scale.x / 4.0, center.y - scale.y / 2.0);
                                let top_anchor = Point::new(center.x, center.y - scale.y / 2.0);
                                let bottom = Point::new(center.x, center.y + scale.y / 2.0);
                                let right_anchor = Point::new(center.x + scale.x / 4.0, center.y - scale.y / 2.0);
                                let top_right = Point::new(center.x + scale.x / 2.0, center.y - scale.y / 2.0);
                                Path::new(|path| {
                                    path.move_to(top_left);
                                    path.bezier_curve_to(left_anchor, top_anchor, bottom);
                                    path.bezier_curve_to(top_anchor, right_anchor, top_right);
                                })
                            },
                            BandType::HighShelf => |_value, center, scale| {
                                let top_right = Point::new(center.x + scale.x / 2.0, center.y - scale.y / 2.0);
                                let top_anchor = Point::new(center.x, center.y - scale.y / 2.0);
                                let left = Point::new(center.x - scale.x / 2.0, center.y);
                                let bottom_anchor = Point::new(center.x, center.y + scale.y / 2.0);
                                let bottom_right = center + Vector::new(scale.x / 2.0, scale.y / 2.0);
                                Path::new(|path| {
                                    path.move_to(top_right);
                                    path.bezier_curve_to(top_anchor, center, left);
                                    path.bezier_curve_to(center, bottom_anchor, bottom_right);
                                })
                            },
                            BandType::LowShelf => |_value, center, scale| {
                                let top_left = center - Vector::new(scale.x / 2.0, scale.y / 2.0);
                                let top_anchor = Point::new(center.x, center.y - scale.y / 2.0);
                                let right = Point::new(center.x + scale.x / 2.0, center.y);
                                let bottom_anchor = Point::new(center.x, center.y + scale.y / 2.0);
                                let bottom_left = Point::new(center.x - scale.x / 2.0, center.y + scale.y / 2.0);
                                Path::new(|path| {
                                    path.move_to(top_left);
                                    path.bezier_curve_to(top_anchor, center, right);
                                    path.bezier_curve_to(center, bottom_anchor, bottom_left);
                                })
                            },
                            BandType::HighPass => |_value, center, scale| {
                                let top_right = Point::new(center.x + scale.x / 2.0, center.y - scale.y / 2.0);
                                let top_anchor = Point::new(center.x, center.y - scale.y / 2.0);
                                let left_anchor = Point::new(center.x - scale.x / 4.0, center.y - scale.y / 2.0);
                                let bottom_left = Point::new(center.x - scale.x / 2.0, center.y + scale.y / 2.0);
                                Path::new(|path| {
                                    path.move_to(top_right);
                                    path.bezier_curve_to(top_anchor, left_anchor, bottom_left);
                                })
                            },
                            BandType::LowPass => |_value, center, scale| {
                                let top_left = Point::new(center.x - scale.x / 2.0, center.y - scale.y / 2.0);
                                let top_anchor = Point::new(center.x, center.y - scale.y / 2.0);
                                let right_anchor = Point::new(center.x + scale.x / 4.0, center.y - scale.y / 2.0);
                                let bottom_right = Point::new(center.x + scale.x / 2.0, center.y + scale.y / 2.0);
                                Path::new(|path| {
                                    path.move_to(top_left);
                                    path.bezier_curve_to(top_anchor, right_anchor, bottom_right);
                                })
                            },
                        },
                        stroke: Stroke::default().with_color(self.theme.foreground),
                        fill: None,
                    }),

                    ..<Theme as param_toggle::StyleSheet>::style(&self.theme)
                }
            }

            fn active(&self) -> param_toggle::Style {
                param_toggle::Style {
                    button: Some(ButtonStyle {
                        stroke: Stroke::default().with_color(self.color),

                        ..self.style().button.unwrap_or(<Theme as param_toggle::StyleSheet>::active(&self.theme).button.unwrap_or_default())
                    }),

                    ..self.style()
                }
            }
        }
        BandShapeToggle {
            theme: self,
            shape,
            color,
        }
    }

    pub fn power_button<'a>(self) -> impl Into<Box<dyn param_toggle::StyleSheet + 'a>> {
        struct PowerButton {
            theme: Theme,
        }
        impl param_toggle::StyleSheet for PowerButton {
            fn style(&self) -> param_toggle::Style {
                param_toggle::Style {
                    label_placement: None,
                    button: Some(ButtonStyle {
                        draw_path: |_value, center, scale| {
                            let scale = f32::min(scale.x, scale.y);
                            let radius = scale * 0.4;
                            Path::new(|path| {
                                path.arc(Arc {
                                    center: center + Vector::new(0.0, scale / 2.0 - radius),
                                    radius,
                                    start_angle: -FRAC_PI_2 + FRAC_PI_8,
                                    end_angle: 3.0 * FRAC_PI_2 - FRAC_PI_8,
                                });
                                path.move_to(center - Vector::new(0.0, scale / 2.0));
                                path.line_to(center);
                            })
                        },
                        stroke: Stroke::default().with_color(self.theme.foreground.lerp_to_inverse(0.5)),
                        fill: None,
                    }),

                    ..<Theme as param_toggle::StyleSheet>::style(&self.theme)
                }
            }

            fn active(&self) -> param_toggle::Style {
                param_toggle::Style {
                    button: Some(ButtonStyle {
                        stroke: Stroke::default().with_color(self.theme.foreground),

                        ..self.style().button.unwrap_or(<Theme as param_toggle::StyleSheet>::active(&self.theme).button.unwrap_or_default())
                    }),

                    ..self.style()
                }
            }
        }
        PowerButton {
            theme: self,
        }
    }

    pub fn headphones_button<'a>(self) -> impl Into<Box<dyn param_toggle::StyleSheet + 'a>> {
        struct HeadphonesButton {
            theme: Theme,
        }
        impl param_toggle::StyleSheet for HeadphonesButton {
            fn style(&self) -> param_toggle::Style {
                param_toggle::Style {
                    label_placement: None,
                    button: Some(ButtonStyle {
                        draw_path: |_value, center, scale| {
                            let scale = f32::min(scale.x, scale.y);
                            Path::new(|path| {
                                let left_ear_bottom = center + Vector::new(-scale * 0.4, scale * 0.5);
                                let right_ear_bottom = center + Vector::new(scale * 0.4, scale * 0.5);
                                let left_ear_top = left_ear_bottom - Vector::new(0.0, scale * 0.4);
                                let right_ear_top = right_ear_bottom - Vector::new(0.0, scale * 0.4);
                                let left_ear_control = left_ear_bottom - Vector::new(scale * 0.15, scale * 0.2);
                                let right_ear_control = right_ear_top + Vector::new(scale * 0.15, scale * 0.2);
                                let headband_left = left_ear_bottom - Vector::new(0.0, scale * 0.5);
                                let headband_right = right_ear_bottom - Vector::new(0.0, scale * 0.5);
                                let headband_left_control = left_ear_bottom - Vector::new(0.0, scale);
                                let headband_right_control = right_ear_bottom - Vector::new(0.0, scale);
                                path.move_to(left_ear_top);
                                path.quadratic_curve_to(left_ear_control, left_ear_bottom);
                                path.line_to(headband_left);
                                path.bezier_curve_to(
                                    headband_left_control,
                                    headband_right_control,
                                    headband_right,
                                );
                                path.line_to(right_ear_bottom);
                                path.quadratic_curve_to(right_ear_control, right_ear_top);
                                path.line_to(headband_right);
                                path.bezier_curve_to(
                                    headband_right_control,
                                    headband_left_control,
                                    headband_left,
                                );
                                path.line_to(left_ear_top);
                            })
                        },
                        stroke: Stroke::default().with_color(self.theme.foreground.lerp_to_inverse(0.5)),
                        fill: Some(self.theme.foreground.lerp_to_inverse(0.5).into()),
                    }),

                    ..<Theme as param_toggle::StyleSheet>::style(&self.theme)
                }
            }

            fn active(&self) -> param_toggle::Style {
                param_toggle::Style {
                    button: Some(ButtonStyle {
                        stroke: Stroke::default().with_color(self.theme.foreground),
                        fill: Some(self.theme.foreground.into()),

                        ..self.style().button.unwrap_or(<Theme as param_toggle::StyleSheet>::active(&self.theme).button.unwrap_or_default())
                    }),

                    ..self.style()
                }
            }
        }
        HeadphonesButton {
            theme: self,
        }
    }
}

impl spectrum::StyleSheet for Theme {
    fn style(&self) -> spectrum::Style {
        spectrum::Style {
            font: self.font,
            text_size: self.text_size,
            text_color: self.foreground,
            
            shape: Shape::Wave(WaveStyle {
                stroke: Some(Stroke::default().with_color(self.foreground)),
                fill: Some(self.foreground.with_alpha(0.5).into()),
            }),
            
            ..spectrum::Style::default()
        }
    }
}

impl Theme {
    pub fn spectrum<'a>(self, color: Color, alignment: spectrum::Alignment) -> impl Into<Box<dyn spectrum::StyleSheet + 'a>> {
        struct Spectrum {
            theme: Theme,
            color: Color,
            alignment: spectrum::Alignment,
        }
        impl spectrum::StyleSheet for Spectrum {
            fn style(&self) -> spectrum::Style {
                spectrum::Style {
                    shape: Shape::Wave(WaveStyle {
                        stroke: Some(Stroke::default().with_color(self.color)),
                        fill: Some(self.color.lerp_to(self.theme.background, 0.5).into()),
                    }),
                    alignment: self.alignment,

                    ..<Theme as spectrum::StyleSheet>::style(&self.theme)
                }
            }
        }
        Spectrum {
            theme: self,
            color,
            alignment,
        }
    }
}

impl equalizer::StyleSheet for Theme {
    fn style(&self) -> equalizer::Style {
        equalizer::Style {
            font: self.font,
            text_size: self.text_size,
            text_color: self.foreground,
            
            line: Stroke::default().with_color(self.foreground),
            fill: self.foreground.with_alpha(0.01).into(),
            
            band_line: Stroke::default(),
            band_fill: Color::default().with_alpha(0.01).into(),
            band_colors: vec![self.orange, self.yellow, self.green, self.cyan, self.blue, self.purple, self.magenta],
            
            node_style: Default::default(),
        }
    }
}