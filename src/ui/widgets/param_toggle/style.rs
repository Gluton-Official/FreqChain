use std::ops::Sub;
use nih_plug_iced::canvas::{Fill, Path, Stroke};
use nih_plug_iced::{Color, Font, Point, Vector};
use crate::ui::ColorUtils;

#[derive(Debug, Clone)]
pub struct Style {
    pub font: Font,
    pub text_size: u16,
    pub text_color: Color,
    pub text_padding: f32,

    pub label_placement: Option<Placement>,
    
    pub button: Option<ButtonStyle>,
}

#[derive(Debug, Clone)]
pub enum Placement {
    Above,
    Inside,
    Below,
}

#[derive(Debug, Clone)]
pub struct ButtonStyle {
    pub draw_path: fn(value: f32, center: Point, scale: Vector) -> Path,
    pub stroke: Stroke<'static>,
    pub fill: Option<Fill>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            font: Font::default(),
            text_size: 12,
            text_color: Color::BLACK,
            text_padding: 4.0,

            label_placement: Some(Placement::Inside),

            button: Some(ButtonStyle::default()),
        }
    }
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self {
            draw_path: |_value, center, scale| {
                Path::new(|path| {
                    path.rectangle(center.sub(Vector::new(0.5 * scale.x, 0.5 * scale.y)).into(), scale.into())
                })
            },
            stroke: Stroke::default().with_color(Color::BLACK.with_alpha(0.5)),
            fill: Some(Color::WHITE.into()),
        }
    }
}

pub trait StyleSheet {
    fn style(&self) -> Style;
    fn hovered(&self) -> Style { self.style() }
    fn active(&self) -> Style { self.hovered() }
    fn disabled(&self) -> Style { self.style() }
}

struct DefaultStyle;

impl StyleSheet for DefaultStyle {
    fn style(&self) -> Style {
        Style::default()
    }
    
    fn hovered(&self) -> Style { 
        Style {
            button: Some(ButtonStyle {
                fill: Some(Color::from_rgb(0.9, 0.9, 0.9).into()),
                
                ..self.style().button.unwrap_or_default()
            }),
            
            ..self.style()
        }
    }

    fn active(&self) -> Style {
        Style {
            button: Some(ButtonStyle {
                fill: Some(Color::from_rgb(0.75, 0.75, 0.75).into()),

                ..self.style().button.unwrap_or_default()
            }),

            ..self.style()
        }
    }

    fn disabled(&self) -> Style {
        Style {
            text_color: Color::BLACK.with_alpha(0.5),
            
            ..self.style()
        }
    }
}

impl<'a> Default for Box<dyn StyleSheet + 'a> {
    fn default() -> Self { Box::new(DefaultStyle) }
}

impl<'a, T> From<T> for Box<dyn StyleSheet + 'a>
where
    T: 'a + StyleSheet,
{
    fn from(style: T) -> Self {
        Box::new(style)
    }
}