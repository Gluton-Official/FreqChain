//! Content shares the full layout space.

use nih_plug_iced::{event, layout, mouse, overlay, renderer, Clipboard, Element, Event, Layout, Length, Padding, Point, Rectangle, Shell, Size, Widget};
use nih_plug_iced::backend::Renderer;

/// A container that allows content to share the full layout space.
#[allow(missing_debug_implementations)]
pub struct Group<'a, Message> {
    padding: Padding,
    width: Length,
    height: Length,
    children: Vec<Element<'a, Message>>,
}

impl<'a, Message> Group<'a, Message> {
    /// Creates an empty [`Group`].
    pub fn new() -> Self {
        Self::with_children(Vec::new())
    }

    /// Creates a [`Group`] with the given elements.
    pub fn with_children(
        children: Vec<Element<'a, Message>>,
    ) -> Self {
        Group {
            padding: Padding::ZERO,
            width: Length::Shrink,
            height: Length::Shrink,
            children,
        }
    }

    /// Sets the [`Padding`] of the [`Group`].
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the width of the [`Group`].
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`Group`].
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Adds an [`Element`] to the [`Group`].
    pub fn push<E>(mut self, child: E) -> Self
    where
        E: Into<Element<'a, Message>>,
    {
        self.children.push(child.into());
        self
    }
}

impl<'a, Message> Widget<Message, Renderer> for Group<'a, Message> {
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits
            .width(self.width)
            .height(self.height);

        let limits = limits
            .loose()
            .width(self.width)
            .height(self.height)
            .pad(self.padding);

        let size = limits.resolve(Size::ZERO);
        
        let children = self.children.iter().map(|child| {
            let mut node = child.layout(renderer, &limits);
            node.move_to(Point::new(
                self.padding.left.into(),
                self.padding.top.into(),
            ));
            node
        }).collect::<Vec<_>>();

        layout::Node::with_children(size.pad(self.padding), children)
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        self.children
            .iter_mut()
            .zip(layout.children())
            .map(|(child, layout)| {
                child.on_event(
                    event.clone(),
                    layout,
                    cursor_position,
                    renderer,
                    clipboard,
                    shell,
                )
            })
            .fold(event::Status::Ignored, event::Status::merge)
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.children
            .iter()
            .zip(layout.children())
            .map(|(child, layout)| {
                child.mouse_interaction(
                    layout,
                    cursor_position,
                    viewport,
                    renderer,
                )
            })
            .max()
            .unwrap_or_default()
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) {
        for (child, layout) in self.children.iter().zip(layout.children()) {
            child.draw(renderer, style, layout, cursor_position, viewport);
        }
    }

    fn overlay(
        &mut self,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'_, Message, Renderer>> {
        self.children
            .iter_mut()
            .zip(layout.children())
            .filter_map(|(child, layout)| {
                child.overlay(layout, renderer)
            })
            .next()
    }
}

impl<'a, Message> From<Group<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(group: Group<'a, Message>) -> Element<'a, Message> {
        Element::new(group)
    }
}
        
