#![allow(dead_code)]

use nih_plug_iced::{Clipboard, Event};
use std::borrow::{Borrow, BorrowMut};
use atomic_refcell::AtomicRefMut;
use nih_plug_iced::{layout, text_input, Background, Color, Font, Layout, Length, Point, Shell, Size, TextInput, Vector, Widget};
use nih_plug_iced::backend::Renderer;
use nih_plug_iced::event::Status;
use nih_plug_iced::text::Renderer as TextRenderer;
use nih_plug_iced::text_input::State;
use crate::ui::ColorUtils;

/// Create a temporary [`TextInput`] hooked up to [`State::text_input_value`] and outputting
/// [`TextInputMessage`] messages and do something with it. This can be used to
pub(super) fn draw_value_input<R>(
    mut text_input_state: AtomicRefMut<State>,
    font: Font,
    text_size: u16,
    style: TextInputStyle,
    cursor_position: Point,
    layout: Layout,
    mut renderer: R,
    current_value: &str,
)
where
    R: BorrowMut<Renderer>,
{
    text_input_state.focus();
    
    let text_width = renderer.borrow().measure_width(current_value, text_size, font);
    let text_input = TextInput::new(&mut text_input_state, "", current_value, TextInputMessage::Value)
        .font(font)
        .size(text_size)
        .width(Length::Units(text_width.ceil() as u16))
        .style(style)
        .on_submit(TextInputMessage::Submit);

    // Make sure to not draw over the border, and center the text
    let offset_node = layout::Node::with_children(
        Size {
            width: text_width,
            height: text_size as f32,
        },
        vec![layout::Node::new(layout.bounds().size())],
    );
    let offset_layout = Layout::with_offset(
        Vector {
            x: layout.bounds().center_x() - (text_width / 2.0),
            y: layout.position().y,
        },
        &offset_node,
    );

    text_input.draw(renderer.borrow_mut(), offset_layout, cursor_position, None);
}

/// Create a temporary [`TextInput`] hooked up to [`State::text_input_value`] and outputting
/// [`TextInputMessage`] messages and do something with it. This can be used to
pub(super) fn get_value_input_status<R>(
    mut text_input_state: AtomicRefMut<State>,
    font: Font,
    text_size: u16,
    style: TextInputStyle,
    cursor_position: Point,
    layout: Layout,
    renderer: R,
    current_value: &str,
    event: Event,
    clipboard: &mut dyn Clipboard,
    text_input_shell: &mut Shell<TextInputMessage>,
) -> Status
where
    R: Borrow<Renderer>,
{
    text_input_state.focus();

    let text_width = renderer.borrow().measure_width(current_value, text_size, font);
    let mut text_input = TextInput::new(&mut text_input_state, "", current_value, TextInputMessage::Value)
        .font(font)
        .size(text_size)
        .width(Length::Units(text_width.ceil() as u16))
        .style(style)
        .on_submit(TextInputMessage::Submit);

    let offset_node = layout::Node::with_children(
        Size {
            width: text_width,
            height: text_size as f32,
        },
        vec![layout::Node::new(layout.bounds().size())],
    );
    let offset_layout = Layout::with_offset(
        Vector {
            x: layout.bounds().center_x() - (text_width / 2.0),
            y: layout.position().y,
        },
        &offset_node,
    );

    text_input.on_event(
        event,
        offset_layout,
        cursor_position,
        renderer.borrow(),
        clipboard,
        text_input_shell,
    )
}

/// An internal message for handling output from the embedded [`TextInput`] widget.
#[derive(Debug, Clone)]
pub(super) enum TextInputMessage {
    /// A new value was entered in the text input dialog.
    Value(String),
    /// Enter was pressed.
    Submit,
}

#[derive(Debug, Clone)]
pub struct TextInputStyle {
    pub background: Background,
    pub border_radius: f32,
    pub border_width: f32,
    pub border_color: Color,
    pub value_color: Color,
    pub placeholder_color: Color,
    pub selection_color: Color,
}

impl Default for TextInputStyle {
    fn default() -> Self {
        Self {
            background: Color::WHITE.into(),
            border_radius: 0.0,
            border_width: 1.0,
            border_color: Color::BLACK,
            value_color: Color::BLACK,
            placeholder_color: Color::WHITE.with_alpha(0.5),
            selection_color: Color::WHITE.with_alpha(0.5),
        }
    }
}

impl text_input::StyleSheet for TextInputStyle {
    fn active(&self) -> text_input::Style { self.into() }
    fn focused(&self) -> text_input::Style { self.active() }
    fn placeholder_color(&self) -> Color { self.placeholder_color }
    fn value_color(&self) -> Color { self.value_color }
    fn selection_color(&self) -> Color { self.selection_color }
}

impl From<&TextInputStyle> for text_input::Style {
    fn from(style: &TextInputStyle) -> Self {
        Self {
            background: style.background,
            border_radius: style.border_radius,
            border_width: style.border_width,
            border_color: style.border_color,
        }
    }
}
