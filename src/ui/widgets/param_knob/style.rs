use std::f32::consts::PI;
use nih_plug_iced::canvas::{Fill, Path, Stroke};
use nih_plug_iced::{Color, Font, Point};
use crate::ui::ColorUtils;
use crate::ui::widgets::value_input::TextInputStyle;

pub const DEFAULT_TURN_DISTANCE: f32 = 5.0 / 3.0 * PI;

#[derive(Debug, Clone)]
pub struct Style {
    /// The distance knob can be turned in radians, centered upward
    pub turn_distance: f32,
    
    pub font: Font,
    pub text_size: u16,
    pub text_color: Color,

    pub label_placement: Option<Placement>,
    pub value_placement: Option<Placement>,

    pub knob: Option<KnobStyle>,
    pub pointer: Option<PointerStyle>,
    pub track: Option<TrackStyle>,

    pub major_tick_marks: Option<TickMarkStyle>,
    pub minor_tick_marks: Option<TickMarkStyle>,

    pub text_input: TextInputStyle,
}

#[derive(Debug, Clone)]
pub enum Placement {
    Above,
    Below,
}

#[derive(Debug, Clone)]
pub struct KnobStyle {
    pub draw_path: fn(center: Point, rotation: f32, scale: f32) -> Path,
    pub stroke: Stroke<'static>,
    pub fill: Option<Fill>,
}

#[derive(Debug, Clone)]
pub struct PointerStyle {
    pub draw_path: fn(center: Point, rotation: f32, scale: f32) -> Path,
    pub stroke: Stroke<'static>,
    pub fill: Option<Fill>,
}

#[derive(Debug, Clone)]
pub struct TrackStyle {
    pub filled_stroke: Stroke<'static>,
    pub unfilled_stroke: Stroke<'static>,
}

#[derive(Debug, Clone)]
pub struct TickMarkStyle {
    pub draw_path: fn(center: Point, rotation: f32, scale: f32) -> Path,
    pub stroke: Stroke<'static>,
    pub values: Vec<f32>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            turn_distance: DEFAULT_TURN_DISTANCE,
            
            font: Font::default(),
            text_size: 12,
            text_color: Color::BLACK,
            
            label_placement: Some(Placement::Below),
            value_placement: Some(Placement::Above),
            
            knob: Some(KnobStyle::default()),
            pointer: Some(PointerStyle::default()),
            track: Some(TrackStyle::default()),

            major_tick_marks: None,
            minor_tick_marks: None,

            text_input: TextInputStyle::default(),
        }
    }
}

impl Default for KnobStyle {
    fn default() -> Self {
        Self {
            draw_path: |center, _rotation, scale| {
                Path::new(|path| {
                    path.circle(center, scale * 0.9)
                })
            },
            stroke: Stroke::default(),
            fill: Some(Color::WHITE.into()),
        }
    }
}

impl Default for PointerStyle {
    fn default() -> Self {
        Self {
            draw_path: |center, rotation, scale| {
                Path::line(center, Point {
                    x: center.x + rotation.cos() * 0.9 * scale,
                    y: center.y + rotation.sin() * 0.9 * scale,
                })
            },
            stroke: Stroke::default(),
            fill: None,
        }
    }
}

impl Default for TrackStyle {
    fn default() -> Self {
        Self {
            filled_stroke: Stroke::default(),
            unfilled_stroke: Stroke::default().with_color(Color::BLACK.with_alpha(0.25)),
        }
    }
}

impl Default for TickMarkStyle {
    fn default() -> Self {
        Self {
            draw_path: |center, rotation, scale| {
                Path::line(
                    Point {
                        x: center.x + rotation.cos() * 0.9 * scale,
                        y: center.y + rotation.sin() * 0.9 * scale,
                    },
                    Point {
                        x: center.x + rotation.cos() * scale,
                        y: center.y + rotation.sin() * scale,
                    }
                )
            },
            stroke: Stroke::default(),
            values: Vec::new(),
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