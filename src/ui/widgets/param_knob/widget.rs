//! A knob that integrates with NIH-plug's [`Param`] types.

use crate::ui::widgets::drag::{DragState, DragTrait};
use crate::ui::widgets::param_knob::style::StyleSheet;
use crate::ui::widgets::param_knob::Placement;
use atomic_refcell::AtomicRefCell;
use nih_plug::prelude::Param;
use nih_plug_iced::alignment::{Horizontal, Vertical};
use nih_plug_iced::backend::Renderer;
use nih_plug_iced::canvas::path::Arc;
use nih_plug_iced::canvas::{Path, Text};
use nih_plug_iced::layout::Limits;
use nih_plug_iced::renderer::Style as RendererStyle;
use nih_plug_iced::text::Renderer as TextRenderer;
use nih_plug_iced::widgets::ParamMessage;
use nih_plug_iced::{canvas, event, keyboard, layout, mouse, touch, Clipboard, Element, Event, Font, Layout, Length, Point, Rectangle, Shell, Size, Widget};
use std::f32::consts::FRAC_PI_2;
use nih_plug_iced::mouse::ScrollDelta;

const SCROLL_SENSITIVITY: f32 = 0.05;
const GRANULAR_SCROLL_MULTIPLIER: f32 = 0.25;

/// A knob that integrates with NIH-plug's [`Param`] types.
pub struct ParamKnob<'a, P: Param> {
    state: &'a mut State,

    param: &'a P,
    
    width: Length,
    height: Length,
    text_size: Option<u16>,
    font: Font,
    anchor: Anchor,
    label: String,

    style_sheet: Box<dyn StyleSheet + 'a>,
}

#[derive(Clone, Debug)]
pub enum Anchor {
    Min,
    Max,
    Center
}

/// State for a [`ParamKnob`].
#[derive(Debug, Default)]
pub struct State {
    keyboard_modifiers: keyboard::Modifiers,

    /// Tracks the starting y-coordinate and normalized value when dragging, including the start coordinate
    /// and normalized value when granular dragging. [None] when not dragging.
    drag_state: AtomicRefCell<Option<DragState>>,

    /// Track clicks for double clicks.
    last_click: Option<mouse::Click>,

    // /// State for the text input overlay that will be shown when this widget is alt+clicked.
    // text_input_state: AtomicRefCell<widget::text_input::State>,
    // /// The text that's currently in the text input. If this is set to `None`, then the text input
    // /// is not visible.
    // text_input_value: Option<String>,
}

impl<'a, P: Param> Widget<ParamMessage, Renderer> for ParamKnob<'a, P> {
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
        shell: &mut Shell<'_, ParamMessage>,
    ) -> event::Status {
        // TODO: fix keyboard input
        // The presence of a value in `self.state.text_input_value` indicates that the field should
        // be focused. The field handles defocusing by itself
        // let text_input_status = self.get_text_input_status(&event, layout, cursor_position, renderer, clipboard, shell);
        // if text_input_status == event::Status::Captured {
        //     return event::Status::Captured;
        // }

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
        _style: &RendererStyle,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        
        let is_hovering = bounds.contains(cursor_position);

        let style = if self.state.drag_state.borrow().is_some() /*|| self.state.text_input_value.is_some() */ {
            self.style_sheet.active()
        } else if is_hovering {
            self.style_sheet.hovered()
        } else {
            self.style_sheet.style()
        };
       
        let text_height = style.text_size as f32 + style.text_padding;
        let mut knob_offset_y = 0.0;
        let mut knob_available_height = bounds.height;
        match style.label_placement {
            Some(Placement::Above) => {
                knob_offset_y += text_height;
                knob_available_height -= text_height;
            },
            Some(Placement::Below) => {
                knob_available_height -= text_height;
            },
            _ => {}
        }
        match style.value_placement {
            Some(Placement::Above) => {
                knob_offset_y += text_height;
                knob_available_height -= text_height;
            },
            Some(Placement::Below) => {
                knob_available_height -= text_height;
            },
            _ => {}
        }
        
        let knob_diameter = f32::min(bounds.width, knob_available_height).max(0.0);
        let knob_radius = knob_diameter / 2.0;
        let knob_bounds = Rectangle::new(
            Point::new(bounds.x + bounds.width / 2.0 - knob_radius, bounds.y + knob_offset_y + knob_available_height / 2.0 - knob_radius),
            Size::new(knob_diameter, knob_diameter)
        );
        
        let normalized_value = self.param.modulated_normalized_value();

        let up_angle = 3.0 * FRAC_PI_2;
        let knob_min_rotation = up_angle - style.turn_distance / 2.0;
        let knob_rotation = knob_min_rotation + style.turn_distance * normalized_value;

        if let Some(track_style) = style.track {
            let mut frame = canvas::Frame::new(viewport.size());

            let background_path = Path::new(|path| {
                path.arc(Arc {
                    center: knob_bounds.center(),
                    radius: knob_radius,
                    start_angle: knob_min_rotation,
                    end_angle: knob_min_rotation + style.turn_distance
                });
            });
            let value_path = Path::new(|path| {
                path.arc(Arc {
                    center: knob_bounds.center(),
                    radius: knob_radius,
                    start_angle: match self.anchor {
                        Anchor::Min => knob_min_rotation,
                        Anchor::Max => knob_min_rotation + style.turn_distance,
                        Anchor::Center => up_angle,
                    },
                    end_angle: knob_rotation,
                });
            });

            frame.stroke(&background_path, track_style.unfilled_stroke);
            frame.stroke(&value_path, track_style.filled_stroke);

            renderer.draw_primitive(frame.into_geometry().into_primitive());
        }

        if let Some(knob_style) = style.knob {
            let mut frame = canvas::Frame::new(viewport.size());

            let draw_path = knob_style.draw_path;
            let knob_path = draw_path(knob_bounds.center(), knob_rotation, knob_radius);

            frame.stroke(&knob_path, knob_style.stroke);

            if let Some(knob_fill) = knob_style.fill {
                frame.fill(&knob_path, knob_fill);
            }

            renderer.draw_primitive(frame.into_geometry().into_primitive());
        }

        if let Some(pointer_style) = style.pointer {
            let mut frame = canvas::Frame::new(viewport.size());

            let draw_path = pointer_style.draw_path;
            let pointer_path = draw_path(knob_bounds.center(), knob_rotation, knob_radius);

            frame.stroke(&pointer_path, pointer_style.stroke);

            if let Some(pointer_fill) = pointer_style.fill {
                frame.fill(&pointer_path, pointer_fill);
            }
            
            renderer.draw_primitive(frame.into_geometry().into_primitive());
        }
        
        if let Some(label_placement) = style.label_placement {
            let mut frame = canvas::Frame::new(viewport.size());
            
            let label_position = match label_placement {
                Placement::Above => knob_bounds.y - style.text_padding,
                Placement::Below => knob_bounds.y + knob_bounds.height + style.text_padding
            };

            let label = Text {
                content: self.label.clone(),
                position: Point::new(bounds.center_x(), label_position),
                color: style.text_color,
                size: style.text_size as f32,
                font: style.font,
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: match label_placement {
                    Placement::Above => Vertical::Bottom,
                    Placement::Below => Vertical::Top,
                },
            };

            frame.fill_text(label);
            
            renderer.draw_primitive(frame.into_geometry().into_primitive());
        }
        
        if let Some(value_placement) = style.value_placement {
            let mut frame = canvas::Frame::new(viewport.size());

            let value_position = match value_placement {
                Placement::Above => knob_bounds.y - style.text_padding,
                Placement::Below => knob_bounds.y + knob_bounds.height + style.text_padding
            };

            let value = Text {
                content: self.param.to_string(),
                position: Point::new(bounds.center_x(), value_position),
                color: style.text_color,
                size: style.text_size as f32,
                font: style.font,
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: match value_placement {
                    Placement::Above => Vertical::Bottom,
                    Placement::Below => Vertical::Top,
                },
            };

            frame.fill_text(value);

            renderer.draw_primitive(frame.into_geometry().into_primitive());
        }

        // if let Some(input_value) = &self.state.text_input_value {
        //     draw_value_input(
        //         self.state.text_input_state.borrow_mut(),
        //         self.font,
        //         self.text_size.unwrap_or(renderer.default_size()),
        //         style.text_input,
        //         cursor_position,
        //         layout,
        //         renderer,
        //         input_value,
        //     );
        // }
    }
}

impl<'a, P: Param> ParamKnob<'a, P> {
    /// Creates a new [`ParamKnob`] for the given parameter.
    pub fn new(state: &'a mut State, param: &'a P) -> Self {
        Self {
            state,

            param,

            width: Length::Units(30),
            height: Length::Units(30),
            text_size: None,
            font: <Renderer as TextRenderer>::Font::default(),
            anchor: Anchor::Min,

            label: param.name().into(),

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

    pub fn anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn style(mut self, style_sheet: impl Into<Box<dyn StyleSheet + 'a>>) -> Self {
        self.style_sheet = style_sheet.into();
        self
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

    // fn get_text_input_status(
    //     &mut self,
    //     event: &Event,
    //     layout: Layout,
    //     cursor_position: Point,
    //     renderer: &Renderer,
    //     clipboard: &mut dyn Clipboard,
    //     shell: &mut Shell<ParamMessage>,
    // ) -> event::Status {
    //     if let Some(current_value) = &self.state.text_input_value {
    //         let event = event.clone();
    //         let mut messages = Vec::new();
    //         let mut text_input_shell = Shell::new(&mut messages);
    //         let status = get_value_input_status(
    //             self.state.text_input_state.borrow_mut(),
    //             self.font,
    //             self.text_size.unwrap_or(renderer.default_size()),
    //             self.style_sheet.style().text_input,
    //             cursor_position,
    //             layout,
    //             renderer,
    //             current_value,
    //             event,
    //             clipboard,
    //             &mut text_input_shell,
    //         );
    //
    //         // Pressing escape will unfocus the text field, so we should propagate that change in
    //         // our own model
    //         if self.state.text_input_state.borrow().is_focused() {
    //             for message in messages {
    //                 match message {
    //                     TextInputMessage::Value(s) => self.state.text_input_value = Some(s),
    //                     TextInputMessage::Submit => {
    //                         if let Some(normalized_value) = self
    //                             .state
    //                             .text_input_value
    //                             .as_ref()
    //                             .and_then(|s| self.param.string_to_normalized_value(s))
    //                         {
    //                             shell.publish(ParamMessage::BeginSetParameter(self.param.as_ptr()));
    //                             self.set_normalized_value(shell, normalized_value);
    //                             shell.publish(ParamMessage::EndSetParameter(self.param.as_ptr()));
    //                         }
    //
    //                         // Defocus the text input widget again
    //                         self.state.text_input_value = None;
    //                     }
    //                 }
    //             }
    //         } else {
    //             self.state.text_input_value = None;
    //         }
    //
    //         status
    //     } else {
    //         event::Status::Ignored
    //     }
    // }

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
                    // if self.state.keyboard_modifiers.alt() {
                    //     // Open text input
                    //     // Cancel drag for text entry
                    //     let mut drag_state = self.state.drag_state.borrow_mut();
                    //     *drag_state = None;
                    //
                    //     let mut text_input_state = self.state.text_input_state.borrow_mut();
                    //     self.state.text_input_value = Some(self.param.to_string());
                    //     text_input_state.move_cursor_to_end();
                    //     text_input_state.select_all();
                    // } else
                    if self.state.keyboard_modifiers.command()
                        || matches!(click.kind(), mouse::click::Kind::Double)
                    {
                        // Reset to default value
                        // Don't start dragging after double-clicking
                        let mut drag_state = self.state.drag_state.borrow_mut();
                        *drag_state = None;

                        shell.publish(ParamMessage::BeginSetParameter(self.param.as_ptr()));
                        self.set_normalized_value(shell, self.param.default_normalized_value());
                        shell.publish(ParamMessage::EndSetParameter(self.param.as_ptr()));
                    } else if self.state.keyboard_modifiers.shift() {
                        // precise dragging
                        shell.publish(ParamMessage::BeginSetParameter(self.param.as_ptr()));

                        let mut drag_state = self.state.drag_state.borrow_mut();
                        *drag_state = Some(DragState::new_and_start_granular(cursor_position.y, self.param.modulated_normalized_value()));
                    } else {
                        // drag
                        shell.publish(ParamMessage::BeginSetParameter(self.param.as_ptr()));

                        let mut drag_state = self.state.drag_state.borrow_mut();
                        *drag_state = Some(DragState::new(cursor_position.y, self.param.modulated_normalized_value()));
                    }

                    return Some(event::Status::Captured);
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. } | touch::Event::FingerLost { .. }) => {
                let mut drag_state = self.state.drag_state.borrow_mut();
                if drag_state.is_some() {
                    shell.publish(ParamMessage::EndSetParameter(self.param.as_ptr()));

                    *drag_state = None;

                    return Some(event::Status::Captured);
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) | Event::Touch(touch::Event::FingerMoved { .. }) => {
                let mut drag_state = self.state.drag_state.borrow_mut();
                if let Some(drag_state) = drag_state.as_mut() {
                    if self.state.keyboard_modifiers.shift() {
                        drag_state.start_granular(cursor_position.y);
                    } else {
                        drag_state.stop_granular(cursor_position.y);
                    }

                    self.set_normalized_value(shell, drag_state.value(bounds.y..(bounds.y + bounds.height), cursor_position.y));

                    return Some(event::Status::Captured);
                }
            }
            Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                if bounds.contains(cursor_position) {
                    let scroll_delta = match delta {
                        ScrollDelta::Lines { y, .. } | ScrollDelta::Pixels { y, .. } => y,
                    };
                    let mut scroll_sensitivity = SCROLL_SENSITIVITY;
                    if self.state.keyboard_modifiers.shift() {
                        scroll_sensitivity *= GRANULAR_SCROLL_MULTIPLIER;
                    }
                    let normalized_scroll_distance = scroll_delta * scroll_sensitivity;

                    shell.publish(ParamMessage::BeginSetParameter(self.param.as_ptr()));
                    self.set_normalized_value(shell, self.param.modulated_normalized_value() + normalized_scroll_distance);
                    shell.publish(ParamMessage::EndSetParameter(self.param.as_ptr()));
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

impl<'a, P: Param> From<ParamKnob<'a, P>> for Element<'a, ParamMessage> {
    fn from(widget: ParamKnob<'a, P>) -> Self {
        Element::new(widget)
    }
}