use nih_plug_iced::{Color, Font};
use nih_plug_iced::canvas::{Fill, LineCap, LineDash, LineJoin, Stroke};

#[derive(Debug, Clone)]
pub struct Style {
    pub font: Font,
    pub text_size: u16,
    pub text_color: Color,
    
    pub shape: Shape,
    
    pub alignment: Alignment,
}

#[derive(Debug, Clone, Copy)]
pub enum Alignment {
    Top,
    Bottom,
}

#[derive(Debug, Clone)]
pub enum Shape {
    Bar(BarStyle),
    Wave(WaveStyle),
}

#[derive(Debug, Clone)]
pub enum BarStyle {
    // Box(BoxStyle),
    Line(LineStyle),
}

// #[derive(Debug, Clone)]
// pub struct BoxStyle {
//     /// Fills space if [None]
//     pub width: Option<f32>,
//     pub gap: f32,
//     pub color: Color,
//     pub fill: bool,
//     pub border: Option<BorderStyle>,
// }
// 
// #[derive(Debug, Clone)]
// pub struct BorderStyle {
//     /// Uses [BoxStyle::color] if [None]
//     pub color: Option<Color>,
//     pub width: f32,
//     pub line_join: LineJoin,
//     pub line_cap: LineCap,
//     pub line_dash: LineDash<'static>,
// }

#[derive(Debug, Clone)]
pub struct LineStyle {
    pub stroke: Stroke<'static>,
}

#[derive(Debug, Clone)]
pub struct WaveStyle {
    pub stroke: Option<Stroke<'static>>,
    pub fill: Option<Fill>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            font: Font::default(),
            text_size: 12,
            text_color: Color::BLACK,
            shape: Shape::Bar(BarStyle::Line(LineStyle::default())),
            alignment: Alignment::Bottom,
        }
    }
}

// impl Default for BoxStyle {
//     fn default() -> Self {
//         Self {
//             width: None,
//             gap: 0.0,
//             color: Color::BLACK.into(),
//             fill: true,
//             border: None,
//         }
//     }
// }

impl Default for LineStyle {
    fn default() -> Self {
        Self {
            stroke: Stroke::default(),
        }
    }
}

pub trait StyleSheet {
    fn style(&self) -> Style;
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
