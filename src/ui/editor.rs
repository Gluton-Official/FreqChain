use crate::freqchain::FreqChainParams;
use crate::ui::theme::FreqChainTheme;
use crate::ui::theme::Theme;
use crate::ui::themeable::Themeable;
use crate::ui::widgets::{param_knob, param_slider};
use crate::ui::widgets::param_knob::{Anchor, ParamKnob};
use crate::ui::{ColorUtils, PaddingExt};
use crate::FreqChain;
use nih_plug::prelude::*;
use nih_plug_iced::{assets, Padding, Vector};
use nih_plug_iced::create_iced_editor;
use nih_plug_iced::executor;
use nih_plug_iced::widgets::ParamMessage;
use nih_plug_iced::Alignment;
use nih_plug_iced::Color;
use nih_plug_iced::Column;
use nih_plug_iced::Command;
use nih_plug_iced::Element;
use nih_plug_iced::IcedEditor;
use nih_plug_iced::IcedState;
use nih_plug_iced::Length;
use nih_plug_iced::Row;
use nih_plug_iced::Rule;
use nih_plug_iced::Text;
use nih_plug_iced::WindowQueue;
use nih_plug_iced::alignment;
use std::sync::Arc;
use nih_plug_iced::canvas::Path;
use crate::ui::widgets::param_slider::ParamSlider;
use crate::ui::widgets::param_toggle::ParamToggle;

const EDITOR_WIDTH: u32 = 192;
const EDITOR_HEIGHT: u32 = 572;
const FPS: f32 = 60.0;

pub(crate) fn default_state() -> Arc<IcedState> {
    IcedState::from_size(EDITOR_WIDTH, EDITOR_HEIGHT)
}

pub(crate) fn create(
    params: Arc<FreqChainParams>,
    sample_rate: Arc<AtomicF32>,
    editor_state: Arc<IcedState>,
) -> Option<Box<dyn Editor>> {
    create_iced_editor::<FreqChainEditor>(editor_state, (params, sample_rate))
}

pub struct FreqChainEditor {
    params: Arc<FreqChainParams>,
    sample_rate: Arc<AtomicF32>,

    theme: Theme,

    context: Arc<dyn GuiContext>,

    sidechain_gain_state: param_slider::State,
    sidechain_detail_state: param_knob::State,
    sidechain_precision_state: param_knob::State,
}

/// Messages to be sent to the editor UI
#[derive(Debug, Clone, Copy)]
pub enum Message {
    ParamUpdate(ParamMessage),
}

impl IcedEditor for FreqChainEditor {
    type Executor = executor::Default;
    type Message = Message;
    type InitializationFlags = (
        Arc<FreqChainParams>,
        Arc<AtomicF32>, // sample rate
    );

    fn new(
        (params, sample_rate): Self::InitializationFlags,
        context: Arc<dyn GuiContext>,
    ) -> (Self, Command<Self::Message>) {
        let editor = FreqChainEditor {
            params,
            sample_rate,

            theme: FreqChainTheme::dark(),

            context,

            sidechain_gain_state: param_slider::State::default(),
            sidechain_detail_state: param_knob::State::default(),
            sidechain_precision_state: param_knob::State::default(),
        };

        (editor, Command::none())
    }

    fn context(&self) -> &dyn GuiContext {
        self.context.as_ref()
    }

    fn update(&mut self, _window: &mut WindowQueue, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::ParamUpdate(message) => self.handle_param_message(message),
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let author = Text::new(FreqChain::VENDOR.to_ascii_uppercase())
            .apply_theme(self.theme)
            .width(Length::FillPortion(1))
            .horizontal_alignment(alignment::Horizontal::Left)
            .vertical_alignment(alignment::Vertical::Center);
        let version = Text::new(format!("v{}", FreqChain::VERSION))
            .apply_theme(self.theme)
            .width(Length::FillPortion(1))
            .horizontal_alignment(alignment::Horizontal::Right)
            .vertical_alignment(alignment::Vertical::Center);

        let title = Text::new(FreqChain::NAME)
            .apply_theme(self.theme)
            .font(assets::NOTO_SANS_REGULAR)
            .size(48)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center);

        let header = Column::new()
            .align_items(Alignment::Center)
            .width(Length::Fill)
            .spacing(2)
            .push(
                Row::new()
                    .align_items(Alignment::Fill)
                    .push(author)
                    .push(version)
            )
            .push(title);

        let frequency_sidechain_label = Column::with_children(
            "FREQUENCY SIDECHAIN".chars().map(|char| {
                Text::new(char)
                    .apply_theme(self.theme)
                    .color(self.theme.foreground.with_alpha(0.5))
                    .height(10.into())
                    .into()
            }).collect())
            .align_items(Alignment::Center)
            .padding(Padding::right(14));

        let sidechain_gain = ParamSlider::new(
            &mut self.sidechain_gain_state,
            &self.params.sidechain_input.gain
        )
            .label("Gain")
            .apply_theme(self.theme)
            .width(26.into())
            .height(Length::Fill)
            .map(Message::ParamUpdate);

        // TODO: change mono_processing to stereo_mode enum
        let mono_toggle = ParamToggle::new(
            &self.params.mono_processing,
        )
            .label("Mono")
            .associated_value(true)
            .style(self.theme.mono_toggle(true))
            .width(30.into())
            .height(20.into())
            .map(Message::ParamUpdate);
        let stereo_toggle = ParamToggle::new(
            &self.params.mono_processing,
        )
            .label("Stereo")
            .associated_value(false)
            .style(self.theme.mono_toggle(false))
            .width(30.into())
            .height(20.into())
            .map(Message::ParamUpdate);

        let detail = ParamKnob::new(
            &mut self.sidechain_detail_state,
            &self.params.frequency_sidechain.detail
        )
            .apply_theme(self.theme)
            .width(Length::Fill)
            .height(Length::FillPortion(1))
            .map(Message::ParamUpdate);
        let precision = ParamKnob::new(
            &mut self.sidechain_precision_state,
            &self.params.frequency_sidechain.precision
        )
            .apply_theme(self.theme)
            .width(Length::Fill)
            .height(Length::FillPortion(1))
            .alignment(Anchor::Center)
            .map(Message::ParamUpdate);

        let frequency_sidechain = Row::new()
            .width(Length::Fill)
            .max_height(190)
            .push(frequency_sidechain_label)
            .push(sidechain_gain)
            .push(Row::new()
                .padding(Padding::left(14))
                .align_items(Alignment::Center)
                .width(Length::FillPortion(1))
                .height(Length::Fill)
                .push(Column::with_children(vec![mono_toggle, stereo_toggle]))
            )
            .push(
                Column::new()
                    .align_items(Alignment::Fill)
                    .width(Length::FillPortion(3))
                    .height(Length::Fill)
                    .spacing(4)
                    .push(detail)
                    .push(precision)
            );

        Column::new()
            .align_items(Alignment::Center)
            .width(Length::Shrink)
            .padding(8)
            .spacing(12)
            .push(header)
            .push(Rule::horizontal(1).apply_theme(self.theme))
            .push(frequency_sidechain)
            .push(Rule::horizontal(1).apply_theme(self.theme))
            .into()
    }

    fn background_color(&self) -> Color {
        self.theme.background
    }
}
