use std::{
    marker::PhantomData,
    sync::{atomic::Ordering, Arc, Mutex},
};

use nih_plug::{
    nih_debug_assert,
    prelude::{AtomicF32, FloatRange},
    util,
};
use nih_plug_iced::canvas::LineJoin;
use nih_plug_iced::{
    backend::Renderer,
    canvas,
    canvas::Path,
    layout, renderer,
    renderer::{Renderer as GraphicsRenderer, Style},
    Color, Element, Layout, Length, Point, Rectangle, Size, Widget,
};

use crate::audio_processing::spectrum::SpectrumOutput;

const BORDER_WIDTH: f32 = 1.0;

pub struct Spectrum<Message> {
    spectrum_buffer: Arc<Mutex<SpectrumOutput>>,
    sample_rate: Arc<AtomicF32>,
    frequency_range: FloatRange,

    width: Length,
    height: Length,

    /// Iced requires a message type even though we don't need any messages
    _phantom: PhantomData<Message>,
}

pub enum Normalization {
    Linear,
    Logarithmic,
}

impl<Message> Spectrum<Message> {
    pub fn new(
        sidechain_spectrum: Arc<Mutex<SpectrumOutput>>,
        sample_rate: Arc<AtomicF32>,
        normalization: Normalization,
    ) -> Self {
        Self {
            spectrum_buffer: sidechain_spectrum,
            sample_rate,
            frequency_range: match normalization {
                Normalization::Linear => FloatRange::Linear {
                    min: 5.0,
                    max: 20_000.0,
                },
                Normalization::Logarithmic => FloatRange::Skewed {
                    min: 5.0,
                    max: 20_000.0,
                    factor: FloatRange::skew_factor(-2.5),
                },
            },

            width: Length::Fill,
            height: Length::Fill,

            _phantom: PhantomData,
        }
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }
}

impl<Message> Widget<Message, Renderer> for Spectrum<Message>
where
    Message: Clone,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(&self, _renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        let limits = limits.width(self.width).height(self.height);
        let size = limits.resolve(Size::ZERO);

        layout::Node::new(size)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        _style: &Style,
        layout: Layout<'_>,
        _cursor_position: Point,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let borderless_bounds = Rectangle {
            x: bounds.x + BORDER_WIDTH,
            y: bounds.y + BORDER_WIDTH,
            width: bounds.width - (BORDER_WIDTH * 2.0),
            height: bounds.height - (BORDER_WIDTH * 2.0),
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border_color: Color::BLACK,
                border_width: BORDER_WIDTH,
                border_radius: 0.0,
            },
            Color::TRANSPARENT,
        );

        let mut spectrum_buffer = self.spectrum_buffer.lock().unwrap();
        let spectrum_buffer = spectrum_buffer.read();
        // The highest frequency that can be represented given the sample rate,
        // see [https://en.wikipedia.org/wiki/Nyquist_frequency]
        let nyquist = self.sample_rate.load(Ordering::Relaxed) / 2.0;

        let spectrum_lines = Path::new(|path| {
            for (bin_index, magnitude) in spectrum_buffer.iter().enumerate() {
                let frequency = (bin_index as f32 / spectrum_buffer.len() as f32) * nyquist;

                // maybe don't clamp but jut drop?
                let t = self.frequency_range.normalize(frequency);

                // scale magnitude so 0dBFS is 80% of widget height and -80dBFS is no height
                nih_debug_assert!(*magnitude >= 0.0);
                let magnitude_db = util::gain_to_db(*magnitude);
                let height = ((magnitude_db + 80.0) / 100.0).clamp(0.0, 1.0);

                path.move_to(Point::new(
                    borderless_bounds.x + borderless_bounds.width * t,
                    borderless_bounds.y + borderless_bounds.height * (1.0 - height),
                ));
                path.line_to(Point::new(
                    borderless_bounds.x + borderless_bounds.width * t,
                    borderless_bounds.y + borderless_bounds.height,
                ))
            }
        });

        let mut frame = canvas::Frame::new(viewport.size());
        frame.stroke(
            &spectrum_lines,
            canvas::Stroke::default().with_color(Color::BLACK).with_width(1.0),
        );
        renderer.draw_primitive(frame.into_geometry().into_primitive());
    }
}

impl<'a, Message> From<Spectrum<Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(widget: Spectrum<Message>) -> Self {
        Element::new(widget)
    }
}
