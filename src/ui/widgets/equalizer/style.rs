use crate::ui::ColorUtils;
use nih_plug_iced::canvas::{Fill, LineDash, Stroke};
use nih_plug_iced::{Color, Font};

#[derive(Debug, Clone)]
pub struct Style {
    pub font: Font,
    pub text_size: u16,
    pub text_color: Color,
    
    pub line: Stroke<'static>,
    pub fill: Fill,

    /// Horizontal line marking 0db
    pub baseline: Option<Stroke<'static>>,
    /// Vertical lines marking select frequencies
    pub frequency_gridlines: Option<FrequencyGridlinesStyle>,
    
    /// Uses respective band color, preserves alpha
    pub band_line: Stroke<'static>,
    /// Uses respective band color, preserves alpha
    pub band_fill: Fill,
    pub band_colors: Vec<Color>,

    /// Uses respective band color, preserves alpha
    pub node_style: NodeStyle,
}

#[derive(Debug, Clone)]
pub struct NodeStyle {
    /// Uses respective band color, preserves alpha
    pub stroke: Option<Stroke<'static>>,
    /// Uses respective band color, preserves alpha
    pub fill: Option<Fill>,
    /// Uses respective band color, preserves alpha
    pub active_ring_stroke: Option<Stroke<'static>>,
}

#[derive(Debug, Clone)]
pub struct FrequencyGridlinesStyle {
    pub stroke: Stroke<'static>,
    pub frequency_values: Vec<f32>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            font: Font::default(),
            text_size: 12,
            text_color: Color::BLACK,

            line: Stroke::default().with_width(2.0),
            fill: Color::BLACK.with_alpha(0.01).into(),

            baseline: Some(Stroke {
                line_dash: LineDash {
                    segments: &[4.0; 2],

                    ..LineDash::default()
                },

                ..Stroke::default()
            }),
            frequency_gridlines: Some(FrequencyGridlinesStyle::default()),
            
            band_line: Stroke::default().with_width(2.0),
            band_fill: Color::BLACK.with_alpha(0.01).into(),
            band_colors: vec![
                Color::from_rgb8(255, 83, 0), // orange
                Color::from_rgb8(255, 255, 0), // yellow
                Color::from_rgb8(114, 255, 33), // green
                Color::from_rgb8(0, 255, 255), // cyan
                Color::from_rgb8(0, 155, 233), // blue
                Color::from_rgb8(136, 33, 255), // purple
                Color::from_rgb8(225, 20, 153), // magenta
            ],
            
            node_style: NodeStyle::default(),
        }
    }
}

impl Default for NodeStyle {
    fn default() -> Self {
        Self {
            stroke: Some(Stroke::default()),
            fill: Some(Color::BLACK.with_alpha(0.5).into()),
            active_ring_stroke: Some(Stroke::default().with_color(Color::BLACK.with_alpha(0.5))),
        }
    }
}

impl Default for FrequencyGridlinesStyle {
    fn default() -> Self {
        Self {
            stroke: Stroke::default(),
            // increments the leading digit by 1
            frequency_values: (1..).flat_map(|power| {
                let base = 10_f32.powi(power);
                (1..=9).map(move |i| i as f32 * base)
            })
                .skip_while(|&v| v < 20.0)
                .take_while(|&v| v <= 20_000.0)
                .collect(),
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
