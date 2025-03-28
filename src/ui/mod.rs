use nih_plug_iced::Color;

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