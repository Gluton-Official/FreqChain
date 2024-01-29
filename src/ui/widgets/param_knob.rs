use std::borrow::Borrow;
use std::f32::consts::{FRAC_PI_2, FRAC_PI_6, TAU};

use atomic_refcell::AtomicRefCell;
use nih_plug::prelude::Param;
use nih_plug_iced::{
    backend::Renderer,
    canvas::{self, path::Arc, Path},
    event, keyboard, layout,
    layout::Limits,
    mouse,
    renderer::{self, Renderer as GraphicsRenderer, Style},
    text::Renderer as TextRenderer,
    touch, widget,
    widgets::{util, ParamMessage},
    Background, Clipboard, Color, Element, Event, Font, Layout, Length, Point, Rectangle, Shell, Size, TextInput,
    Vector, Widget,
};

/// When shift+dragging a parameter, one pixel dragged corresponds to this much change in the
/// normalized parameter.
const GRANULAR_DRAG_MULTIPLIER: f32 = 0.1;

// Knob's border width
const BORDER_WIDTH: f32 = 1.0;

/// A knob that integrates with NIH-plug's [`Param`] types.
pub struct ParamKnob<'a, P: Param> {
    state: &'a mut State,

    param: &'a P,

    size: Length,
    text_size: Option<u16>,
    font: Font,
}

/// State for a [`ParamKnob`].
#[derive(Debug, Default)]
pub struct State {
    keyboard_modifiers: keyboard::Modifiers,
    /// Will be set to `true` if we're dragging the parameter. Resetting the parameter or entering a
    /// text value should not initiate a drag.
    drag_active: bool,
    /// We keep track of the start coordinate and normalized value holding down Shift while dragging
    /// for higher precision dragging. This is a `None` value when granular dragging is not active.
    granular_drag_start_y_value: Option<(f32, f32)>,
    /// Track clicks for double clicks.
    last_click: Option<mouse::Click>,

    /// State for the text input overlay that will be shown when this widget is alt+clicked.
    text_input_state: AtomicRefCell<widget::text_input::State>,
    /// The text that's currently in the text input. If this is set to `None`, then the text input
    /// is not visible.
    text_input_value: Option<String>,
}

/// An internal message for handling output from the embedded [`TextInput`] widget.
#[derive(Debug, Clone)]
enum TextInputMessage {
    /// A new value was entered in the text input dialog.
    Value(String),
    /// Enter was pressed.
    Submit,
}

/// The default text input style with the border removed.
struct TextInputStyle;

impl widget::text_input::StyleSheet for TextInputStyle {
    fn active(&self) -> widget::text_input::Style {
        widget::text_input::Style {
            background: Background::Color(Color::TRANSPARENT),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        }
    }

    fn focused(&self) -> widget::text_input::Style {
        self.active()
    }

    fn placeholder_color(&self) -> Color {
        Color::from_rgb(0.7, 0.7, 0.7)
    }

    fn value_color(&self) -> Color {
        Color::from_rgb(0.3, 0.3, 0.3)
    }

    fn selection_color(&self) -> Color {
        Color::from_rgb(0.8, 0.8, 0.8)
    }
}

impl<'a, P: Param> ParamKnob<'a, P> {
    /// Creates a new [`ParamKnob`] for the given parameter.
    pub fn new(state: &'a mut State, param: &'a P) -> Self {
        Self {
            state,

            param,

            size: Length::Units(30),
            text_size: None,
            font: <Renderer as TextRenderer>::Font::default(),
        }
    }

    /// Sets the size of the [`ParamKnob`].
    pub fn size(mut self, size: Length) -> Self {
        self.size = size;
        self
    }

    /// Sets the text size of the [`ParamKnob`].
    pub fn text_size(mut self, size: u16) -> Self {
        self.text_size = Some(size);
        self
    }

    /// Sets the font of the [`ParamKnob`].
    pub fn font(mut self, font: Font) -> Self {
        self.font = font;
        self
    }

    /// Create a temporary [`TextInput`] hooked up to [`State::text_input_value`] and outputting
    /// [`TextInputMessage`] messages and do something with it. This can be used to
    fn with_text_input<T, R, F>(&self, layout: Layout, renderer: R, current_value: &str, f: F) -> T
    where
        F: FnOnce(TextInput<'_, TextInputMessage>, Layout, R) -> T,
        R: Borrow<Renderer>,
    {
        let mut text_input_state = self.state.text_input_state.borrow_mut();
        text_input_state.focus();

        let text_size = self.text_size.unwrap_or_else(|| renderer.borrow().default_size());
        let text_width = renderer.borrow().measure_width(current_value, text_size, self.font);
        let text_input = TextInput::new(&mut text_input_state, "", current_value, TextInputMessage::Value)
            .font(self.font)
            .size(text_size)
            .width(Length::Units(text_width.ceil() as u16))
            .style(TextInputStyle)
            .on_submit(TextInputMessage::Submit);

        // Make sure to not draw over the border, and center the text
        let offset_node = layout::Node::with_children(
            Size {
                width: text_width,
                height: layout.bounds().size().height - (BORDER_WIDTH * 2.0),
            },
            vec![layout::Node::new(layout.bounds().size())],
        );
        let offset_layout = Layout::with_offset(
            Vector {
                x: layout.bounds().center_x() - (text_width / 2.0),
                y: layout.position().y + BORDER_WIDTH,
            },
            &offset_node,
        );

        f(text_input, offset_layout, renderer)
    }

    /// Set the normalized value for a parameter if that would change the parameter's plain value
    /// (to avoid unnecessary duplicate parameter changes). The begin- and end set parameter
    /// messages need to be sent before calling this function.
    fn set_normalized_value(&self, shell: &mut Shell<'_, ParamMessage>, normalized_value: f32) {
        // This snaps to the nearest plain value if the parameter is stepped in some way.
        let plain_value = self.param.preview_plain(normalized_value);
        let current_plain_value = self.param.modulated_plain_value();
        if plain_value != current_plain_value {
            // For the aforementioned snapping
            let normalized_plain_value = self.param.preview_normalized(plain_value);
            shell.publish(ParamMessage::SetParameterNormalized(
                self.param.as_ptr(),
                normalized_plain_value,
            ));
        }
    }
}

impl<'a, P: Param> Widget<ParamMessage, Renderer> for ParamKnob<'a, P> {
    fn width(&self) -> Length {
        self.size
    }

    fn height(&self) -> Length {
        self.size
    }

    fn layout(&self, _renderer: &Renderer, limits: &Limits) -> layout::Node {
        let limits = limits.width(self.size).height(self.size);
        let size = limits.resolve(Size::ZERO);

        layout::Node::new(size)
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, ParamMessage>,
    ) -> event::Status {
        // The presence of a value in `self.state.text_input_value` indicates that the field should
        // be focused. The field handles defocusing by itself
        let text_input_status = self.get_text_input_status(&event, layout, cursor_position, renderer, clipboard, shell);
        if text_input_status == event::Status::Captured {
            return event::Status::Captured;
        }

        let bounds = layout.bounds();
        let bounds = Rectangle {
            x: bounds.x + BORDER_WIDTH,
            y: bounds.y + BORDER_WIDTH,
            width: bounds.width - (BORDER_WIDTH * 2.0),
            height: bounds.height - (BORDER_WIDTH * 2.0),
        };

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
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let bounds = layout.bounds();
        let is_mouse_over = bounds.contains(cursor_position);

        if is_mouse_over {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        _style: &Style,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let borderless_bounds = Rectangle {
            x: bounds.x + BORDER_WIDTH,
            y: bounds.y + BORDER_WIDTH,
            width: bounds.width - (BORDER_WIDTH * 2.0),
            height: bounds.height - (BORDER_WIDTH * 2.0),
        };
        let is_mouse_over = bounds.contains(cursor_position);

        // Draw circle
        // TODO: use stylesheet
        let background_color = if is_mouse_over || self.state.drag_active || self.state.text_input_value.is_some() {
            Color::new(0.5, 0.5, 0.5, 0.1)
        } else {
            Color::TRANSPARENT
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border_color: Color::BLACK,
                border_width: 0.0,
                border_radius: bounds.width / 2.0,
            },
            background_color,
        );

        // Only draw the text input widget when it is focused.
        // Otherwise, overlay the label with the knob
        if let Some(current_value) = &self.state.text_input_value {
            self.with_text_input(layout, renderer, current_value, |text_input, layout, renderer| {
                text_input.draw(renderer, layout, cursor_position, None)
            })
        } else {
            // Visualize the difference between the current value and the default value if the
            // default value lies somewhere in the middle and the parameter is continuous.
            let current_value = self.param.modulated_normalized_value();
            let default_value = self.param.default_normalized_value();
            let mut start_value = current_value * 0.8;
            let mut end_value = default_value * 0.8;
            if start_value > end_value {
                std::mem::swap(&mut start_value, &mut end_value)
            };

            let value_path = Path::new(|path| {
                path.arc(Arc {
                    center: bounds.center(),
                    radius: borderless_bounds.width / 2.0,
                    start_angle: start_value * TAU + FRAC_PI_6 + FRAC_PI_2,
                    end_angle: end_value * TAU + FRAC_PI_6 + FRAC_PI_2,
                });
            });
            let background_path = Path::new(|path| {
                path.arc(Arc {
                    center: bounds.center(),
                    radius: borderless_bounds.width / 2.0,
                    start_angle: FRAC_PI_6 + FRAC_PI_2,
                    end_angle: 0.8 * TAU + FRAC_PI_6 + FRAC_PI_2,
                });
            });

            let mut frame = canvas::Frame::new(viewport.size());
            frame.stroke(
                &background_path,
                canvas::Stroke::default()
                    .with_color(Color::from_rgba8(0, 0, 0, 0.5))
                    .with_width(1.0),
            );
            frame.stroke(
                &value_path,
                canvas::Stroke::default().with_color(Color::BLACK).with_width(1.0),
            );
            renderer.draw_primitive(frame.into_geometry().into_primitive());
        }
    }
}

impl<'a, P: Param> ParamKnob<'a, P> {
    fn get_text_input_status(
        &mut self,
        event: &Event,
        layout: Layout,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<ParamMessage>,
    ) -> event::Status {
        if let Some(current_value) = &self.state.text_input_value {
            let event = event.clone();
            let mut messages = Vec::new();
            let mut text_input_shell = Shell::new(&mut messages);
            let status = self.with_text_input(layout, renderer, current_value, |mut text_input, layout, renderer| {
                text_input.on_event(
                    event,
                    layout,
                    cursor_position,
                    renderer,
                    clipboard,
                    &mut text_input_shell,
                )
            });

            // Pressing escape will unfocus the text field, so we should propagate that change in
            // our own model
            if self.state.text_input_state.borrow().is_focused() {
                for message in messages {
                    match message {
                        TextInputMessage::Value(s) => self.state.text_input_value = Some(s),
                        TextInputMessage::Submit => {
                            if let Some(normalized_value) = self
                                .state
                                .text_input_value
                                .as_ref()
                                .and_then(|s| self.param.string_to_normalized_value(s))
                            {
                                shell.publish(ParamMessage::BeginSetParameter(self.param.as_ptr()));
                                self.set_normalized_value(shell, normalized_value);
                                shell.publish(ParamMessage::EndSetParameter(self.param.as_ptr()));
                            }

                            // Defocus the text input widget again
                            self.state.text_input_value = None;
                        }
                    }
                }
            } else {
                self.state.text_input_value = None;
            }

            status
        } else {
            event::Status::Ignored
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
                    let click = mouse::Click::new(cursor_position, self.state.last_click);
                    self.state.last_click = Some(click);
                    if self.state.keyboard_modifiers.alt() {
                        // Cancel drag for text entry
                        self.state.drag_active = false;

                        let mut text_input_state = self.state.text_input_state.borrow_mut();
                        self.state.text_input_value = Some(self.param.to_string());
                        text_input_state.move_cursor_to_end();
                        text_input_state.select_all();
                    } else if self.state.keyboard_modifiers.command()
                        || matches!(click.kind(), mouse::click::Kind::Double)
                    {
                        // Don't start dragging after double clicking
                        self.state.drag_active = false;

                        shell.publish(ParamMessage::BeginSetParameter(self.param.as_ptr()));
                        self.set_normalized_value(shell, self.param.default_normalized_value());
                        shell.publish(ParamMessage::EndSetParameter(self.param.as_ptr()));
                    } else if self.state.keyboard_modifiers.shift() {
                        shell.publish(ParamMessage::BeginSetParameter(self.param.as_ptr()));
                        self.state.drag_active = true;

                        // granular drag
                        self.state.granular_drag_start_y_value =
                            Some((cursor_position.y, self.param.modulated_normalized_value()));
                    } else {
                        shell.publish(ParamMessage::BeginSetParameter(self.param.as_ptr()));
                        self.state.drag_active = true;

                        self.set_normalized_value(shell, util::remap_rect_y_coordinate(&bounds, cursor_position.y));
                        self.state.granular_drag_start_y_value = None;
                    }

                    return Some(event::Status::Captured);
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. } | touch::Event::FingerLost { .. }) => {
                if self.state.drag_active {
                    shell.publish(ParamMessage::EndSetParameter(self.param.as_ptr()));

                    self.state.drag_active = false;

                    return Some(event::Status::Captured);
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) | Event::Touch(touch::Event::FingerMoved { .. }) => {
                if self.state.drag_active {
                    if self.state.keyboard_modifiers.shift() {
                        let (drag_start_y, drag_start_value) = *self
                            .state
                            .granular_drag_start_y_value
                            .get_or_insert_with(|| (cursor_position.y, self.param.modulated_normalized_value()));

                        self.set_normalized_value(
                            shell,
                            util::remap_rect_y_coordinate(
                                &bounds,
                                util::remap_rect_y_t(&bounds, drag_start_value)
                                    + (cursor_position.y - drag_start_y) * GRANULAR_DRAG_MULTIPLIER,
                            ),
                        );
                    } else {
                        self.state.granular_drag_start_y_value = None;

                        self.set_normalized_value(shell, util::remap_rect_y_coordinate(&bounds, cursor_position.y));
                    }

                    return Some(event::Status::Captured);
                }
            }
            Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) => {
                self.state.keyboard_modifiers = modifiers;

                // cancel drag
                if self.state.drag_active && self.state.granular_drag_start_y_value.is_some() && !modifiers.shift() {
                    self.state.granular_drag_start_y_value = None;

                    self.set_normalized_value(shell, util::remap_rect_y_coordinate(&bounds, cursor_position.y));
                }

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

impl<'a, P: Param> From<ParamKnob<'a, P>> for Element<'a, ParamMessage> {
    fn from(widget: ParamKnob<'a, P>) -> Self {
        Element::new(widget)
    }
}
