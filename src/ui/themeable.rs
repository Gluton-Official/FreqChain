use nih_plug_iced::{Rule, Text};
use nih_plug::params::Param;
use crate::ui::theme::Theme;
use crate::ui::widgets::param_knob::ParamKnob;
use crate::ui::widgets::param_slider::ParamSlider;

pub trait Themeable {
    fn apply_theme(self, theme: Theme) -> Self;
}

impl Themeable for Text {
    fn apply_theme(self, theme: Theme) -> Self {
        self
            .font(theme.font)
            .size(theme.text_size)
            .color(theme.foreground)
    }
}

impl Themeable for Rule<'_> {
    fn apply_theme(self, theme: Theme) -> Self {
        self.style(theme)
    }
}

impl<P: Param> Themeable for ParamKnob<'_, P> {
    fn apply_theme(self, theme: Theme) -> Self {
        self.style(theme)
    }
}

impl<P: Param> Themeable for ParamSlider<'_, P> {
    fn apply_theme(self, theme: Theme) -> Self {
        self.style(theme)
    }   
}