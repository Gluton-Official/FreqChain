//! A parametric equalizer widget

use crate::modules::equalizer::{BandParams, BandType, EqualizerParams};
use crate::ui::widgets::drag::{DragState2D, DragTrait};
use crate::ui::widgets::equalizer::{FrequencyGridlinesStyle, StyleSheet};
use crate::ui::{ColorUtils, RectangleExt};
use crate::util::biquad_filter::BiquadFilter;
use crate::util::remap::{map_normalized, map_normalized_inv_log10_ranged, map_normalized_ranged, normalize_log10_ranged, normalize_ranged};
use atomic_refcell::AtomicRefCell;
use nih_plug::params::Param;
use nih_plug::util::MINUS_INFINITY_DB;
use nih_plug_iced::backend::Renderer;
use nih_plug_iced::canvas::{Fill, Path};
use nih_plug_iced::layout::Limits;
use nih_plug_iced::mouse::ScrollDelta;
use nih_plug_iced::renderer::Style;
use nih_plug_iced::widgets::util::{remap_rect_x_coordinate, remap_rect_x_t, remap_rect_y_coordinate};
use nih_plug_iced::widgets::ParamMessage;
use nih_plug_iced::{canvas, event, keyboard, layout, mouse, touch, Clipboard, Element, Event, Layout, Length, Point, Rectangle, Shell, Size, Vector, Widget};
use std::f32::consts::TAU;
use std::ops::Range;

const SCROLL_SENSITIVITY: f32 = 0.05;
const GRANULAR_SCROLL_MULTIPLIER: f32 = 0.25;

/// A parametric equalizer widget
pub struct Equalizer<'a, const BANDS: usize> {
    state: &'a mut State,
    
    params: &'a EqualizerParams<BANDS>,
    
    width: Length,
    height: Length,
    node_radius: f32,

    db_range: Range<f32>,
    frequency_range: Range<f32>,
    
    sample_rate: f32,

    style_sheet: Box<dyn StyleSheet + 'a>,
}

#[derive(Debug, Default)]
pub struct State {
    keyboard_modifiers: keyboard::Modifiers,

    /// Tracks the starting y-coordinate and normalized value when dragging, including the start coordinate
    /// and normalized value when granular dragging. [None] when not dragging.
    drag_state: AtomicRefCell<Option<DragState2D>>,

    /// Band node that is currently being edited or hovered
    active_node_index: usize,

    /// Track clicks for double clicks.
    last_click: Option<mouse::Click>,
}

impl State {
    pub fn active_node_index(&self) -> usize {
        self.active_node_index
    }
}

impl<'a, const BANDS: usize> Widget<ParamMessage, Renderer> for Equalizer<'a, BANDS> {
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

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, ParamMessage>
    ) -> event::Status {
        let bounds = layout.bounds();
        if let Some(value) = self.handle_input_event(event, cursor_position, shell, &bounds) {
            return value;
        }

        event::Status::Ignored
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor_position: Point,
        _viewport: &Rectangle,
        _renderer: &Renderer
    ) -> mouse::Interaction {
        let bounds = layout.bounds();
        let is_mouse_over = bounds.contains(cursor_position);
        
        if self.state.drag_state.borrow().is_some() {
            mouse::Interaction::Grabbing
        } else if self.calculate_node_bounds(&bounds).iter().any(|node_bounds| node_bounds.contains(cursor_position)) {
           mouse::Interaction::Grab 
        } else if is_mouse_over {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(&self,
        renderer: &mut Renderer,
        _style: &Style,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle
    ) {
        let bounds = layout.bounds();

        let style = self.style_sheet.style();
        
        let mut frame = canvas::Frame::new(viewport.size());
        frame.with_clip(bounds, |frame| {
            let frame_bounds = Rectangle::new(Point::ORIGIN, bounds.size());
            let node_bounds = self.calculate_node_bounds(&frame_bounds);

            // Baseline
            if let Some(baseline_stroke) = style.baseline {
                let baseline_path = Path::line(
                    Point::new(frame_bounds.x, frame_bounds.center_y()),
                    Point::new(frame_bounds.x + frame_bounds.width, frame_bounds.center_y())
                );
                frame.stroke(&baseline_path, baseline_stroke);
            }

            // Frequency gridlines
            if let Some(FrequencyGridlinesStyle { stroke, frequency_values }) = style.frequency_gridlines {
                let path = Path::new(|path| {
                    for frequency_value in frequency_values {
                        let x = self.frequency_to_x(&frame_bounds, frequency_value);

                        path.move_to(Point::new(x, 0.0));
                        path.line_to(Point::new(x, frame_bounds.height));
                    }
                });
                frame.stroke(&path, stroke);
            }

            // EQ Line
            {
                let points = (0..=frame_bounds.width as usize).map(|i| {
                    let point_x = remap_rect_x_t(&frame_bounds, i as f32 / frame_bounds.width);
                    let point_frequency = self.x_to_frequency(&frame_bounds, point_x);
                    let point_db = self.params.bands.iter()
                        .map(|band| self.calculate_band_gain_db(band, point_frequency))
                        .sum::<f32>();

                    self.frequency_and_db_to_position(&frame_bounds, point_frequency, point_db)
                }).collect::<Vec<_>>();

                let path = Path::new(|path| {
                    path.move_to(points[0]);
                    for i in 1..points.len() {
                        path.line_to(points[i]);
                    }
                });

                frame.stroke(&path, style.line);

                let fill_path = Path::new(|path| {
                    path.move_to(points[0]);
                    for i in 1..points.len() {
                        path.line_to(points[i]);
                    }
                    path.line_to(Point::new(frame_bounds.x + frame_bounds.width, frame_bounds.y + frame_bounds.height));
                    path.line_to(Point::new(frame_bounds.x, frame_bounds.y + frame_bounds.height));
                });
                
                frame.fill(&fill_path, style.fill)
            }

            // Band Line (shown if dragging or hovering)
            if let Some(index) = self.state.drag_state.borrow()
                .map(|_| self.state.active_node_index)
                .or_else(|| {
                    let cursor_position = cursor_position - Vector::new(bounds.x, bounds.y); // offset because of frame clip
                    self.find_hovered_node(node_bounds.as_slice(), cursor_position)
                })
            {
                let band_color = style.band_colors[index % style.band_colors.len()];
                
                let points = (0..=frame_bounds.width as usize).map(|i| {
                    let point_x = remap_rect_x_t(&frame_bounds, i as f32 / frame_bounds.width);
                    let point_frequency = self.x_to_frequency(&frame_bounds, point_x);
                    let point_db = self.calculate_band_gain_db(&self.params.bands[index], point_frequency);

                    self.frequency_and_db_to_position(&frame_bounds, point_frequency, point_db)
                }).collect::<Vec<_>>();

                let path = Path::new(|path| {
                    path.move_to(points[0]);
                    for i in 1..points.len() {
                        path.line_to(points[i]);
                    }
                });

                frame.stroke(&path, style.band_line.with_color(band_color));

                let fill_path = Path::new(|path| {
                    path.move_to(points[0]);
                    for i in 1..points.len() {
                        path.line_to(points[i]);
                    }
                    path.line_to(Point::new(frame_bounds.x + frame_bounds.width, frame_bounds.y + frame_bounds.height));
                    path.line_to(Point::new(frame_bounds.x, frame_bounds.y + frame_bounds.height));
                });

                frame.fill(&fill_path, band_color.with_alpha(style.band_fill.color.a));
            }

            // Nodes
            for (index, node_bounds) in node_bounds.iter().enumerate() {
                let path = Path::circle(node_bounds.center(), self.node_radius);

                let band_color = style.band_colors[index % style.band_colors.len()];

                // Draw node fill if not bypassed
                if !self.params.bands[index].bypass.value() {
                    if let Some(node_fill) = style.node_style.fill {
                        let fill_alpha = node_fill.color.a;
                        frame.fill(&path, Fill {
                            color: band_color.with_alpha(fill_alpha),
                            ..node_fill
                        });
                    }
                }

                if let Some(node_stroke) = style.node_style.stroke {
                    let stroke_alpha = node_stroke.color.a;
                    frame.stroke(&path, node_stroke.with_color(band_color.with_alpha(stroke_alpha)));
                }

                // Show node ring if active
                if index == self.state.active_node_index {
                    if let Some(node_ring_stroke) = style.node_style.active_ring_stroke {
                        let path = Path::circle(node_bounds.center(), self.node_radius * 2.0);
                        let stroke_alpha = node_ring_stroke.color.a;
                        frame.stroke(&path, node_ring_stroke.with_color(band_color.with_alpha(stroke_alpha)))
                    }
                }
            }
        });

        renderer.draw_primitive(frame.into_geometry().into_primitive());
    }
}


impl<'a, const BANDS: usize> Equalizer<'a, BANDS> {
    /// Creates a new [`Equalizer`] for the given equalizer [`State`] and [`EqualizerParams`].
    pub fn new(state: &'a mut State, params: &'a EqualizerParams<BANDS>, sample_rate: f32) -> Self {
        Self {
            state,
            
            params,

            width: Length::Units(30),
            height: Length::Units(30),
            node_radius: 7.0,

            db_range: -18.0..18.0,
            frequency_range: 20.0..20000.0,
            
            sample_rate,

            style_sheet: Default::default(),
        }
    }

    /// Sets the width of the [`Equalizer`].
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`Equalizer`].
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the usable decibel range that band nodes can be moved
    pub fn db_range(mut self, range: Range<f32>) -> Self {
        self.db_range = range;
        self
    }
    
    /// Sets the usable frequency range that band nodes can be moved
    pub fn frequency_range(mut self, range: Range<f32>) -> Self {
        self.frequency_range = range;
        self
    }
    
    /// Sets the sample rate used to calculate visual elements
    pub fn sample_rate(mut self, sample_rate: f32) -> Self {
        self.sample_rate = sample_rate;
        self
    }

    pub fn style(mut self, style_sheet: impl Into<Box<dyn StyleSheet + 'a>>) -> Self {
        self.style_sheet = style_sheet.into();
        self
    }

    fn frequency_to_x(&self, bounds: &Rectangle, frequency: f32) -> f32 {
        remap_rect_x_t(bounds, normalize_log10_ranged(frequency, &self.frequency_range))
    }

    /// Converts an x-coordinate within specified bounds to a corresponding frequency.
    ///
    /// This function takes a normalized x-coordinate within the given `bounds` (represented
    /// by a `Rectangle`) and maps it to a frequency value using an inverse logarithmic scale.
    /// The frequency is calculated based on the `self.frequency_range`.
    fn x_to_frequency(&self, bounds: &Rectangle, x: f32) -> f32 {
        map_normalized_inv_log10_ranged(remap_rect_x_coordinate(bounds, x), &self.frequency_range)
    }

    fn db_to_y(&self, bounds: &Rectangle, gain_db: f32) -> f32 {
        map_normalized(1.0 - normalize_ranged(gain_db, &self.db_range), bounds.y, bounds.y + bounds.height)
    }

    #[allow(unused)]
    fn y_to_db(&self, bounds: &Rectangle, y: f32) -> f32 {
        map_normalized_ranged(1.0 - remap_rect_y_coordinate(bounds, y), &self.db_range)
    }

    fn frequency_and_db_to_position(&self, bounds: &Rectangle, frequency: f32, db: f32) -> Point {
        Point::new(self.frequency_to_x(bounds, frequency), self.db_to_y(bounds, db))
    }

    #[allow(unused)]
    fn position_to_frequency_and_db(&self, bounds: &Rectangle, position: Point) -> (f32, f32) {
        (self.x_to_frequency(bounds, position.x), self.y_to_db(bounds, position.y))
    }

    fn band_node_position(&self, bounds: &Rectangle, band_params: &BandParams) -> Point {
        self.frequency_and_db_to_position(&bounds, band_params.frequency.value(), band_params.db.value())
    }

    fn calculate_node_bounds(&self, bounds: &Rectangle) -> Vec<Rectangle> {
        self.params.bands.iter().map(|band| {
            let node_position = self.band_node_position(&bounds, band);
            let node_size = Vector::new(self.node_radius * 2.0, self.node_radius * 2.0);
            Rectangle::new(node_position - node_size * 0.5, node_size.into())
        }).collect::<Vec<_>>()
    }

    fn find_hovered_node(&self, node_bounds: &[Rectangle], cursor_position: Point) -> Option<usize> {
        node_bounds.iter().enumerate()
            .filter(|(_, bounds)| bounds.inflate(bounds.size().into()).contains(cursor_position)) // double hit area
            .min_by(|(_, bounds_a), (_, bounds_b)| {
                let a_delta = bounds_a.center() - cursor_position;
                let a_distance_sq = a_delta.x.powi(2) + a_delta.y.powi(2);
                let b_delta = bounds_b.center() - cursor_position;
                let b_distance_sq = b_delta.x.powi(2) + b_delta.y.powi(2);
                a_distance_sq.partial_cmp(&b_distance_sq).unwrap()
            })
            .map(|(index, _)| index)
    }

    fn calculate_band_gain_db(&self, band_params: &BandParams, frequency: f32) -> f32 {
        if band_params.bypass.value() {
            return 0.0;
        }

        if matches!(
            band_params.band_type.value(),
            BandType::Peak | BandType::HighShelf | BandType::LowShelf
        ) && band_params.db.value() == 0_f32 {
            return 0.0;
        }

        let filter_coefficients = band_params.calculate_filter(self.sample_rate);
        let BiquadFilter { b0, b1, b2, a1, a2 } = filter_coefficients;

        // Calculate angular frequency (ωT)
        let wt = TAU * frequency / self.sample_rate;
        let cos_wt = wt.cos();
        let sin_wt = wt.sin();
        let cos_2wt = (2.0 * wt).cos();
        let sin_2wt = (2.0 * wt).sin();

        // Calculate the real and imaginary parts of the numerator and denominator for transfer function H(e^(jωT))
        // N = b0 + b1*e^(-jωT) + b2*e^(-2jωT)
        // D = 1 + a1*e^(-jωT) + a2*e^(-2jωT)
        let n_real = b0 + b1 * cos_wt + b2 * cos_2wt;
        let n_imag = -b1 * sin_wt - b2 * sin_2wt;
        let d_real = 1.0 + a1 * cos_wt + a2 * cos_2wt;
        let d_imag = -a1 * sin_wt - a2 * sin_2wt;

        // Calculate squared magnitude
        let n_mag_sq = n_real.powi(2) + n_imag.powi(2);
        let d_mag_sq = d_real.powi(2) + d_imag.powi(2);

        let magnitude = if d_mag_sq < 1e-12 { // denominator is zero
            1e6
        } else if n_mag_sq < 1e-12 { // numerator is zero
            0.0
        } else {
            (n_mag_sq / d_mag_sq).sqrt()
        };

        if magnitude <= 1e-12 {
            MINUS_INFINITY_DB
        } else {
            20.0 * magnitude.log10()
        }
    }

    /// Set the normalized value for a parameter if that would change the parameter's plain value
    /// (to avoid unnecessary duplicate parameter changes). The begin- and end set parameter
    /// messages need to be sent before calling this function.
    fn set_normalized_value<P: Param>(&self, shell: &mut Shell<'_, ParamMessage>, param: &P, normalized_value: f32) {
        // This snaps to the nearest plain value if the parameter is stepped in some way.
        let plain_value = param.preview_plain(normalized_value);
        let current_plain_value = param.modulated_plain_value();
        if plain_value != current_plain_value {
            // For the aforementioned snapping
            let normalized_plain_value = param.preview_normalized(plain_value);
            shell.publish(ParamMessage::SetParameterNormalized(
                param.as_ptr(),
                normalized_plain_value,
            ));
        }
    }

    fn handle_input_event(
        &mut self,
        event: Event,
        cursor_position: Point,
        shell: &mut Shell<ParamMessage>,
        bounds: &Rectangle,
    ) -> Option<event::Status> {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if bounds.contains(cursor_position) {
                    let node_bounds = self.calculate_node_bounds(bounds);
                    if let Some(hovering_node_index) = self.find_hovered_node(node_bounds.as_slice(), cursor_position) {
                        let band_params = &self.params.bands[hovering_node_index];

                        let click = mouse::Click::new(cursor_position, self.state.last_click);
                        self.state.last_click = Some(click);

                        if self.state.keyboard_modifiers.command()
                            || matches!(click.kind(), mouse::click::Kind::Double)
                        {
                            // Reset to default value
                            // Don't start dragging after double-clicking
                            let mut drag_state = self.state.drag_state.borrow_mut();
                            *drag_state = None;

                            shell.publish(ParamMessage::BeginSetParameter(band_params.frequency.as_ptr()));
                            shell.publish(ParamMessage::BeginSetParameter(band_params.db.as_ptr()));
                            self.set_normalized_value(shell, &band_params.frequency, band_params.frequency.default_normalized_value());
                            self.set_normalized_value(shell, &band_params.db, band_params.db.default_normalized_value());
                            shell.publish(ParamMessage::EndSetParameter(band_params.frequency.as_ptr()));
                            shell.publish(ParamMessage::EndSetParameter(band_params.db.as_ptr()));
                        } else if self.state.keyboard_modifiers.shift() {
                            // precise dragging
                            shell.publish(ParamMessage::BeginSetParameter(band_params.frequency.as_ptr()));
                            shell.publish(ParamMessage::BeginSetParameter(band_params.db.as_ptr()));

                            let mut drag_state = self.state.drag_state.borrow_mut();
                            *drag_state = Some(DragState2D::new_and_start_granular(cursor_position,
                                Vector {
                                    x: band_params.frequency.modulated_normalized_value(),
                                    y: band_params.db.modulated_normalized_value(),
                                }
                            ));
                            self.state.active_node_index = hovering_node_index;
                        } else {
                            // drag
                            shell.publish(ParamMessage::BeginSetParameter(band_params.frequency.as_ptr()));
                            shell.publish(ParamMessage::BeginSetParameter(band_params.db.as_ptr()));

                            let mut drag_state = self.state.drag_state.borrow_mut();
                            *drag_state = Some(DragState2D::new(cursor_position,
                                Vector {
                                    x: band_params.frequency.modulated_normalized_value(),
                                    y: band_params.db.modulated_normalized_value()
                                }
                            ));
                            self.state.active_node_index = hovering_node_index;
                        }

                        return Some(event::Status::Captured);
                    }
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. } | touch::Event::FingerLost { .. }) => {
                let mut drag_state = self.state.drag_state.borrow_mut();
                if drag_state.is_some() {
                    let band_params = &self.params.bands[self.state.active_node_index];
                    shell.publish(ParamMessage::EndSetParameter(band_params.frequency.as_ptr()));
                    shell.publish(ParamMessage::EndSetParameter(band_params.db.as_ptr()));

                    *drag_state = None;

                    return Some(event::Status::Captured);
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. })
            | Event::Touch(touch::Event::FingerMoved { .. }) => {
                let mut drag_state = self.state.drag_state.borrow_mut();
                if let Some(drag_state) = drag_state.as_mut() {
                    let band_params = &self.params.bands[self.state.active_node_index];

                    if self.state.keyboard_modifiers.shift() {
                        drag_state.start_granular(cursor_position);
                    } else {
                        drag_state.stop_granular(cursor_position);
                    }

                    let normalized_vector = drag_state.value(*bounds, cursor_position);
                    self.set_normalized_value(shell, &band_params.frequency, normalized_vector.x);
                    self.set_normalized_value(shell, &band_params.db, normalized_vector.y);

                    return Some(event::Status::Captured);
                }
            }
            Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                if bounds.contains(cursor_position) {
                    let node_bounds = self.calculate_node_bounds(bounds);
                    if let Some(hovering_node_index) = self.find_hovered_node(node_bounds.as_slice(), cursor_position) {
                        let band_params = &self.params.bands[hovering_node_index];

                        let scroll_delta = match delta {
                            ScrollDelta::Lines { y, .. } | ScrollDelta::Pixels { y, .. } => -y,
                        };
                        let mut scroll_sensitivity = SCROLL_SENSITIVITY;
                        if self.state.keyboard_modifiers.shift() {
                            scroll_sensitivity *= GRANULAR_SCROLL_MULTIPLIER;
                        }
                        let normalized_scroll_distance = scroll_delta * scroll_sensitivity;

                        shell.publish(ParamMessage::BeginSetParameter(band_params.q.as_ptr()));
                        self.set_normalized_value(shell, &band_params.q, band_params.q.modulated_normalized_value() + normalized_scroll_distance);
                        shell.publish(ParamMessage::EndSetParameter(band_params.q.as_ptr()));
                    }
                }
            }
            Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) => {
                self.state.keyboard_modifiers = modifiers;

                return Some(event::Status::Captured);
            }
            _ => {}
        }

        None
    }

    pub fn map<Message, F>(self, f: F) -> Element<'a, Message>
    where
        Message: 'static,
        F: Fn(ParamMessage) -> Message + 'static,
    {
        Element::from(self).map(f)
    }
}

impl<'a, const BANDS: usize> From<Equalizer<'a, BANDS>> for Element<'a, ParamMessage> {
    fn from(widget: Equalizer<'a, BANDS>) -> Self {
        Element::new(widget)
    }
}
