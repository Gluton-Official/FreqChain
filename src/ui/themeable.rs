use nih_plug_iced::{Container, Rule, Text};
use nih_plug::params::Param;
use crate::ui::theme::Theme;
use crate::ui::widgets::param_knob::ParamKnob;
use crate::ui::widgets::param_slider::ParamSlider;
use crate::ui::widgets::param_toggle::ParamToggle;
use crate::ui::widgets::spectrum::Spectrum;

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

impl<Message> Themeable for Container<'_, Message> {
    fn apply_theme(self, theme: Theme) -> Self {
        self.style(theme)
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

impl<P: Param> Themeable for ParamToggle<'_, P>
where
    P::Plain: Copy
{
    fn apply_theme(self, theme: Theme) -> Self {
        self.style(theme)
    }
}

impl<const CHANNELS: usize, const BINS: usize> Themeable for Spectrum<'_, CHANNELS, BINS> {
    fn apply_theme(self, theme: Theme) -> Self {
        self.style(theme)
    }
}