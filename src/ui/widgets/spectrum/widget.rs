//! An audio spectrum.

use crate::ui::editor::Message;
use crate::ui::widgets::spectrum::{Alignment, BarStyle, Shape, StyleSheet};
use crate::util::remap::{normalize_log10, normalize_ranged};
use atomic_refcell::AtomicRefCell;
use nih_plug::util::{gain_to_db, MINUS_INFINITY_DB};
use nih_plug_iced::backend::Renderer;
use nih_plug_iced::canvas::Path;
use nih_plug_iced::layout::Limits;
use nih_plug_iced::renderer::Style as RendererStyle;
use nih_plug_iced::widgets::util::remap_rect_x_t;
use nih_plug_iced::{canvas, layout, Element, Layout, Length, Point, Rectangle, Size, Widget};
use realfft::num_complex::Complex32;
use std::ops::Range;
use std::sync::Arc;

/// An audio spectrum.
pub struct Spectrum<'a, const CHANNELS: usize, const BINS: usize> {
    state: &'a mut State<CHANNELS, BINS>,

    width: Length,
    height: Length,

    db_range: Range<f32>,
    frequency_range: Range<f32>,

    style_sheet: Box<dyn StyleSheet + 'a>,
}

#[derive(Debug)]
pub struct State<const CHANNELS: usize, const BINS: usize> {
    spectrum_buffer: Arc<AtomicRefCell<triple_buffer::Output<[[Complex32; BINS]; CHANNELS]>>>,
}

impl<const CHANNELS: usize, const BINS: usize> State<CHANNELS, BINS> {
    pub fn new(buffer_out: Arc<AtomicRefCell<triple_buffer::Output<[[Complex32; BINS]; CHANNELS]>>>) -> Self {
        Self {
            spectrum_buffer: buffer_out,
        }
    }
}

impl<'a, Message, const CHANNELS: usize, const BINS: usize> Widget<Message, Renderer> for Spectrum<'a, CHANNELS, BINS> {
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(&self, _renderer: &Renderer, limits: &Limits) -> layout::Node {
        let limits = limits.width(self.width).height(self.height);
        let size = limits.resolve(Size::ZERO);

        layout::Node::new(size)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        _style: &RendererStyle,
        layout: Layout<'_>,
        _cursor_position: Point,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();

        // TODO: display Hz and dB under cursor
        // let is_hovering = bounds.contains(cursor_position);

        let style = self.style_sheet.style();

        let mut spectrum_buffer_ref = self.state.spectrum_buffer.borrow_mut();
        let spectrum_buffer = spectrum_buffer_ref.read();

        let bins = spectrum_buffer[0].len();
        let channels = spectrum_buffer.len();

        let mut frame = canvas::Frame::new(viewport.size());
        match style.shape {
            Shape::Bar(BarStyle::Line(line_style)) => {
                let (y_alignment, centering_multiplier) = match style.alignment {
                    Alignment::Top => (bounds.y, -bounds.height / 2.0),
                    Alignment::Bottom => (bounds.y + bounds.height, bounds.height / 2.0),
                };
                let path = Path::new(|path| {
                    let mut prev_bin_x = bounds.x;

                    for bin_index in 0..bins {
                        // average magnitudes across channels
                        let mut magnitude = 0_f32;
                        for channel_index in 0..channels {
                            magnitude += spectrum_buffer[channel_index][bin_index].norm();
                        }
                        magnitude /= channels as f32;

                        let bin_x = remap_rect_x_t(&bounds, normalize_log10(bin_index as f32 + 2.0, 2.0, bins as f32 + 2.0));
                        let bin_width = bin_x - prev_bin_x;
                        let line_x = prev_bin_x + bin_width / 2.0; // center line
                        prev_bin_x = bin_x;

                        let scaled_magnitude = normalize_ranged(gain_to_db(magnitude), &self.db_range); // scale to min/max dB window
                        let line_end_y = y_alignment - scaled_magnitude * centering_multiplier; // center 0dB

                        path.move_to([line_x, y_alignment].into());
                        path.line_to([line_x, line_end_y.clamp(bounds.y, bounds.y + bounds.height)].into());
                    }
                });
                frame.stroke(&path, line_style.stroke);
            }
            Shape::Wave(wave_style) => {
                frame.with_clip(bounds, |frame| {
                    let bounds = Rectangle::new(Point::ORIGIN, bounds.size()); // redefine bounds to be relative to clip
                    let (y_alignment, centering_multiplier) = match style.alignment {
                        Alignment::Top => (bounds.y, -bounds.height / 2.0),
                        Alignment::Bottom => (bounds.y + bounds.height, bounds.height / 2.0),
                    };
                    let path = Path::new(|path| {
                        let mut prev_bin_x = bounds.x;
                        let mut points = (0..bins).map(|bin_index| {
                            let mut magnitude = 0_f32;
                            for channel_index in 0..channels {
                                magnitude += spectrum_buffer[channel_index][bin_index].norm();
                            }
                            magnitude /= channels as f32;

                            let bin_x = remap_rect_x_t(&bounds, normalize_log10(bin_index as f32 + 2.0, 2.0, bins as f32 + 2.0));
                            let bin_width = bin_x - prev_bin_x;
                            let x_pos = prev_bin_x + bin_width / 2.0; // center point
                            prev_bin_x = bin_x;

                            let scaled_magnitude = normalize_ranged(gain_to_db(magnitude), &self.db_range); // scale to min/max dB window
                            let y_pos = y_alignment - scaled_magnitude * centering_multiplier; // center 0dB

                            Point::new(x_pos, y_pos)
                        }).collect::<Vec<_>>();
                        
                        let scaled_magnitude = normalize_ranged(MINUS_INFINITY_DB, &self.db_range); // scale to min/max dB window
                        let y_pos = y_alignment - scaled_magnitude * centering_multiplier; // center 0dB
                        points.insert(0, [bounds.x, y_pos].into());
                        points.push([bounds.x + bounds.width, y_pos].into());
                        
                        path.move_to(points[0]);
                        for i in 0..points.len() - 1 {
                            let p0 = if i == 0 { points[0] } else { points[i - 1] };
                            let p1 = points[i];
                            let p2 = points[i + 1];
                            let p3 = if i + 2 >= points.len() { points[i + 1] } else { points[i + 2] };

                            let control_a = Point::new(
                                p1.x + (p2.x - p0.x) / 6.0,
                                p1.y + (p2.y - p0.y) / 6.0
                            );
                            let control_b = Point::new(
                                p2.x - (p3.x - p1.x) / 6.0,
                                p2.y - (p3.y - p1.y) / 6.0
                            );
                            
                            path.bezier_curve_to(control_a, control_b, p2);
                        }
                    });
                    
                    if let Some(wave_fill) = wave_style.fill {
                        frame.fill(&path, wave_fill);
                    }
                    
                    if let Some(wave_stroke) = wave_style.stroke {
                        frame.stroke(&path, wave_stroke);
                    }
                });
            }
        }
        renderer.draw_primitive(frame.into_geometry().into_primitive());
    }
}

impl<'a, const CHANNELS: usize, const BINS: usize> Spectrum<'a, CHANNELS, BINS> {
    /// Creates a new [`ParamKnob`] for the given parameter.
    pub fn new(state: &'a mut State<CHANNELS, BINS>) -> Self {
        Self {
            state,

            width: Length::Units(30),
            height: Length::Units(30),

            db_range: -18.0..18.0,
            frequency_range: 20.0..20000.0,

            style_sheet: Default::default(),
        }
    }

    /// Sets the width of the [`ParamKnob`].
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`ParamKnob`].
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the range in decibels that is visible
    pub fn db_range(mut self, range: Range<f32>) -> Self {
        self.db_range = range;
        self
    }

    pub fn style(mut self, style_sheet: impl Into<Box<dyn StyleSheet + 'a>>) -> Self {
        self.style_sheet = style_sheet.into();
        self
    }
}

impl<'a, const CHANNELS: usize, const BINS: usize> From<Spectrum<'a, CHANNELS, BINS>> for Element<'a, Message> {
    fn from(widget: Spectrum<'a, CHANNELS, BINS>) -> Self {
        Element::new(widget)
    }
}
