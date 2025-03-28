use nih_plug_iced::{assets, rule, text_input, Background, Color, Font};
use nih_plug_iced::canvas::Stroke;
use crate::ui::ColorUtils;
use crate::ui::widgets::param_knob;
use crate::ui::widgets::param_knob::{KnobStyle, PointerStyle, TrackStyle};
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
            track: Some(TrackStyle {
                filled_stroke: Stroke::default().with_color(self.foreground),
                unfilled_stroke: Stroke::default().with_color(self.foreground.with_alpha(0.25)),
                
                ..TrackStyle::default()
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
                
                ..self.style().knob.unwrap()
            }),
            
            ..self.style()
        }
    }
}

