use nih_plug_iced::{Color, Padding, Rectangle, Size, Vector};

pub mod editor;
pub mod widgets;
pub mod theme;
mod themeable;

trait ColorUtils {
    fn with_alpha(&self, val: f32) -> Color;
    fn lerp_to(&self, target: Color, val: f32) -> Color;
    fn lerp_to_inverse(&self, val: f32) -> Color;
}

impl ColorUtils for Color {
    fn with_alpha(&self, val: f32) -> Color {
        let mut color = self.clone();
        color.a = val;
        color
    }

    fn lerp_to(&self, target: Color, t: f32) -> Color {
        Color::new(
            (target.r - self.r) * t + self.r,
            (target.g - self.g) * t + self.g,
            (target.b - self.b) * t + self.b,
            (target.a - self.a) * t + self.a,
        )
    }

    fn lerp_to_inverse(&self, t: f32) -> Color {
        self.lerp_to(self.inverse(), t)
    }
}

trait PaddingExt {
    fn top(padding: u16) -> Self;
    fn right(padding: u16) -> Self;
    fn bottom(value: u16) -> Self;
    fn left(padding: u16) -> Self;
}

impl PaddingExt for Padding {
    fn top(value: u16) -> Self {
        let mut padding = Padding::ZERO;
        padding.top = value;
        padding
    }

    fn right(value: u16) -> Self {
        let mut padding = Padding::ZERO;
        padding.right = value;
        padding
    }

    fn bottom(value: u16) -> Self {
        let mut padding = Padding::ZERO;
        padding.bottom = value;
        padding
    }

    fn left(value: u16) -> Self {
        let mut padding = Padding::ZERO;
        padding.left = value;
        padding
    }
}

trait RectangleExt {
    fn inflate(&self, amount: Vector<f32>) -> Self;
}

impl RectangleExt for Rectangle<f32> {
    fn inflate(&self, amount: Vector<f32>) -> Self {
        let inflated_size: Size = (Vector::from(self.size()) + amount).into();
        let adjusted_position = self.center() - Vector::from(inflated_size) * 0.5;
        Rectangle::new(adjusted_position, inflated_size)
    }
}