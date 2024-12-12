use std::sync::Arc;

use nih_plug::prelude::*;
use nih_plug_iced::{alignment, assets, create_iced_editor, executor, widgets::ParamMessage, Alignment, Color, Column, Command, Element, IcedEditor, IcedState, Length, Row, Rule, Text, WindowQueue};

use crate::freqchain::FreqChainParams;
use crate::FreqChain;

const EDITOR_WIDTH: u32 = 192;
const EDITOR_HEIGHT: u32 = 108;

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
    context: Arc<dyn GuiContext>,

    sample_rate: Arc<AtomicF32>,
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
        Arc<AtomicF32>,             // sample rate
    );

    fn new(
        (params, sample_rate): Self::InitializationFlags,
        context: Arc<dyn GuiContext>,
    ) -> (Self, Command<Self::Message>) {
        let editor = FreqChainEditor {
            params,
            context,

            sample_rate,
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
        let text_color = Color::from_rgb(228.0 / 255.0, 228.0 / 255.0, 228.0 / 255.0);

        let author = Text::new(FreqChain::VENDOR.to_ascii_uppercase())
            .font(assets::NOTO_SANS_LIGHT)
            .color(text_color)
            .size(16)
            .width(Length::FillPortion(1))
            .horizontal_alignment(alignment::Horizontal::Left)
            .vertical_alignment(alignment::Vertical::Center);
        let version = Text::new(format!("v{}", FreqChain::VERSION))
            .font(assets::NOTO_SANS_LIGHT)
            .color(text_color)
            .size(16)
            .width(Length::FillPortion(1))
            .horizontal_alignment(alignment::Horizontal::Right)
            .vertical_alignment(alignment::Vertical::Center);

        let title = Text::new(FreqChain::NAME)
            .font(assets::NOTO_SANS_REGULAR)
            .color(text_color)
            .size(48)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center);

        Column::new()
            .align_items(Alignment::Center)
            .width(Length::Shrink)
            .padding(8)
            .push(
                Row::new()
                    .align_items(Alignment::Fill)
                    .push(author)
                    .spacing(4)
                    .push(version)
            )
            .spacing(4)
            .push(title)
            .spacing(8)
            .push(Rule::horizontal(1))
            .into()
    }

    fn background_color(&self) -> Color {
        Color::from_rgb(20.0 / 255.0, 20.0 / 255.0, 20.0 / 255.0)
    }
}
