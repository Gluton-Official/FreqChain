//! A toggle that integrates with NIH-plug's [`Param`] types.

use nih_plug::prelude::Param;

use nih_plug_iced::backend::Renderer;
use nih_plug_iced::text::Renderer as TextRenderer;
use nih_plug_iced::{canvas, event, layout, mouse, renderer, touch, Clipboard, Element, Event, Font, Layout, Length, Point, Rectangle, Shell, Size, Widget};
use nih_plug_iced::alignment::{Horizontal, Vertical};
use nih_plug_iced::canvas::Text;
use nih_plug_iced::widgets::ParamMessage;
use crate::ui::widgets::param_toggle::{Placement, StyleSheet};

/// A toggle that integrates with NIH-plug's [`Param`] types.
pub struct ParamToggle<'a, P: Param>
where
    P::Plain: Copy
{
    param: &'a P,

    width: Length,
    height: Length,
    text_size: Option<u16>,
    font: Font,
    label: String,

    associated_value: Option<P::Plain>,
    set_associated_value_only: bool,

    style_sheet: Box<dyn StyleSheet + 'a>,
}

impl<'a, P: Param> Widget<ParamMessage, Renderer> for ParamToggle<'a, P>
where
    P::Plain: Copy
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

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, ParamMessage>,
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
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();

        let is_hovering = bounds.contains(cursor_position);

        let is_active = self.associated_value
            .map(|value| value == self.param.modulated_plain_value())
            .unwrap_or(self.param.modulated_normalized_value() >= 0.5);

        let style = if is_active {
            self.style_sheet.active()
        } else if is_hovering {
            self.style_sheet.hovered()
        } else {
            self.style_sheet.style()
        };

        let text_height = style.text_size as f32 + style.text_padding;
        let mut toggle_offset_y = 0.0;
        let mut toggle_available_height = bounds.height;
        match style.label_placement {
            Some(Placement::Above) => {
                toggle_offset_y += text_height;
                toggle_available_height -= text_height;
            },
            Some(Placement::Below) => {
                toggle_available_height -= text_height;
            },
            _ => {}
        }

        let toggle_bounds = Rectangle::new(
            Point::new(bounds.x, bounds.y + toggle_offset_y),
            Size::new(bounds.width, toggle_available_height),
        );

        let normalized_value = self.param.modulated_normalized_value();
        
        if let Some(button_style) = style.button {
            let mut frame = canvas::Frame::new(viewport.size());

            let draw_path = button_style.draw_path;
            let button_path = draw_path(normalized_value, toggle_bounds.center(), toggle_bounds.size().into());
            
            frame.stroke(&button_path, button_style.stroke);
            
            if let Some(button_fill) = button_style.fill {
                frame.fill(&button_path, button_fill);
            }
            
            renderer.draw_primitive(frame.into_geometry().into_primitive());
        }
        
        if let Some(label_placement) = style.label_placement {
            let mut frame = canvas::Frame::new(viewport.size());

            let label_position = match label_placement {
                Placement::Above => toggle_bounds.y - style.text_padding,
                Placement::Inside => toggle_bounds.center_y(),
                Placement::Below => toggle_bounds.y + toggle_bounds.height + style.text_padding,
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
                    Placement::Inside => Vertical::Center,
                    Placement::Below => Vertical::Top,
                },
            };

            frame.fill_text(label);

            renderer.draw_primitive(frame.into_geometry().into_primitive());
        }
    }
}

impl<'a, P: Param> ParamToggle<'a, P>
where
    P::Plain: Copy
{
    /// Creates a new [`ParamSlider`] for the given parameter.
    pub fn new(param: &'a P) -> Self {
        Self {
            param,

            width: Length::Units(30),
            height: Length::Units(20),
            text_size: None,
            font: <Renderer as TextRenderer>::Font::default(),
            
            label: param.name().into(),

            associated_value: None,
            set_associated_value_only: false,

            style_sheet: Default::default(),
        }
    }

    /// Sets the width of the [`ParamSlider`].
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`ParamSlider`].
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the text size of the [`ParamSlider`].
    pub fn text_size(mut self, size: u16) -> Self {
        self.text_size = Some(size);
        self
    }

    /// Sets the font of the [`ParamSlider`].
    pub fn font(mut self, font: Font) -> Self {
        self.font = font;
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

    pub fn associated_value(mut self, value: P::Plain) -> Self {
        self.associated_value = Some(value);
        self
    }

    pub fn associated_value_exclusive(mut self, value: P::Plain) -> Self {
        self.associated_value = Some(value);
        self.set_associated_value_only = true;
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

    fn set_plain_value(&self, shell: &mut Shell<'_, ParamMessage>, plain_value: P::Plain) {
        let current_plain_value = self.param.modulated_plain_value();
        if plain_value != current_plain_value {
            let normalized_plain_value = self.param.preview_normalized(plain_value);
            shell.publish(ParamMessage::SetParameterNormalized(
                self.param.as_ptr(),
                normalized_plain_value,
            ))
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
                    if self.associated_value.is_some() && self.set_associated_value_only {
                        if self.param.modulated_plain_value() != self.associated_value.unwrap() {
                            shell.publish(ParamMessage::BeginSetParameter(self.param.as_ptr()));
                            self.set_plain_value(shell, self.associated_value.unwrap());
                            shell.publish(ParamMessage::EndSetParameter(self.param.as_ptr()));

                            return Some(event::Status::Captured)
                        }
                    } else {
                        shell.publish(ParamMessage::BeginSetParameter(self.param.as_ptr()));
                        self.set_normalized_value(shell, 1.0 - self.param.modulated_normalized_value().round());
                        shell.publish(ParamMessage::EndSetParameter(self.param.as_ptr()));

                        return Some(event::Status::Captured)
                    }
                }
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

impl<'a, P: Param> From<ParamToggle<'a, P>> for Element<'a, ParamMessage>
where
    P::Plain: Copy
{
    fn from(widget: ParamToggle<'a, P>) -> Self {
        Element::new(widget)
    }
}
