use nih_plug_iced::canvas::{Fill, Path, Stroke};
use nih_plug_iced::{Color, Font, Point, Vector};
use crate::ui::ColorUtils;
use crate::ui::widgets::value_input::TextInputStyle;

#[derive(Debug, Clone)]
pub struct Style {
    pub font: Font,
    pub text_size: u16,
    pub text_color: Color,
    pub text_padding: f32,
    
    pub orientation: Orientation,
    
    pub label_placement: Option<Placement>,
    pub value_placement: Option<Placement>,

    pub handle: Option<HandleStyle>,
    pub track: Option<TrackStyle>,

    pub major_tick_marks: Option<TickMarkStyle>,
    pub minor_tick_marks: Option<TickMarkStyle>,

    pub text_input: TextInputStyle,
}

#[derive(Debug, Clone)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone)]
pub enum Placement {
    Above,
    Below,
}

#[derive(Debug, Clone)]
pub struct HandleStyle {
    pub draw_path: fn(orientation: Orientation, center: Point, scale: f32) -> Path,
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
    pub draw_path: fn(orientation: Orientation, center: Point, scale: f32) -> Path,
    pub stroke: Stroke<'static>,
    pub values: Vec<f32>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            font: Font::default(),
            text_size: 12,
            text_color: Color::BLACK,
            text_padding: 4.0,
            
            orientation: Orientation::Vertical,
            
            label_placement: Some(Placement::Below),
            value_placement: Some(Placement::Above),
            
            handle: Some(HandleStyle::default()),
            track: Some(TrackStyle::default()),

            major_tick_marks: None,
            minor_tick_marks: None,

            text_input: TextInputStyle::default(),
        }
    }
}

impl Default for HandleStyle {
    fn default() -> Self {
        Self {
            draw_path: |orientation, center, scale| {
                let size = match orientation {
                    Orientation::Vertical => Vector::new(1.0, 1.0 / 3.0) * scale,
                    Orientation::Horizontal => Vector::new(1.0 / 3.0, 1.0) * scale,
                };
                Path::rectangle(
                    Point::new(center.x - size.x / 2.0, center.y - size.y / 2.0),
                    size.into(),
                )
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
            draw_path: |orientation, center, scale| {
                match orientation {
                    Orientation::Vertical => Path::line([center.x - 0.5 * scale, center.y].into(), [center.x + 0.5 * scale, center.y].into()),
                    Orientation::Horizontal => Path::line([center.x, center.y - 0.5 * scale].into(), [center.x, center.y + 0.5 * scale].into())
                }
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