use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::Duration;

use nih_plug::prelude::{AtomicF32, Editor, GuiContext};
use nih_plug::util;
use nih_plug_iced::{Alignment, alignment, assets, Color, Column, Command, create_iced_editor, Element, executor, IcedEditor, IcedState, Length, Space, Text, WindowQueue};
use nih_plug_iced::widgets::{param_slider, ParamMessage, ParamSlider, peak_meter, PeakMeter};

use crate::FreqChainParams;

pub(crate) fn default_state() -> Arc<IcedState> {
    IcedState::from_size(120, 200)
}

pub(crate) fn create(
    params: Arc<FreqChainParams>,
    peak_meter: Arc<AtomicF32>,
    editor_state: Arc<IcedState>,
) -> Option<Box<dyn Editor>> {
    create_iced_editor::<FreqChainEditor>(editor_state, (params, peak_meter))
}

struct FreqChainEditor {
    params: Arc<FreqChainParams>,
    context: Arc<dyn GuiContext>,

    peak_meter: Arc<AtomicF32>,

    gain_slider_state: param_slider::State,
    peak_meter_state: peak_meter::State,
}

/// Messages to be sent to the editor UI
#[derive(Debug, Clone, Copy)]
enum Message {
    ParamUpdate(ParamMessage),
}

impl IcedEditor for FreqChainEditor {
    type Executor = executor::Default;
    type Message = Message;
    type InitializationFlags = (Arc<FreqChainParams>, Arc<AtomicF32>);

    fn new(
        (params, peak_meter): Self::InitializationFlags,
        context: Arc<dyn GuiContext>,
    ) -> (Self, Command<Self::Message>) {
        let editor = FreqChainEditor {
            params,
            context,

            peak_meter,

            gain_slider_state: Default::default(),
            peak_meter_state: Default::default(),
        };

        (editor, Command::none())
    }

    fn context(&self) -> &dyn GuiContext {
        self.context.as_ref()
    }

    fn update(
        &mut self,
        _window: &mut WindowQueue,
        message: Self::Message,
    ) -> Command<Self::Message> {
        match message {
            Message::ParamUpdate(message) => self.handle_param_message(message),
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Column::new()
            .align_items(Alignment::Center)
            .push(
                Text::new("FreqChain GUI")
                    .font(assets::NOTO_SANS_LIGHT)
                    .size(24)
                    .height(50.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Bottom)
            )
            .push(
                Text::new("Gain")
                    .height(20.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center)
            )
            .push(
                ParamSlider::new(&mut self.gain_slider_state, &self.params.gain)
                    .map(Message::ParamUpdate)
            )
            .push(Space::with_height(10.into()))
            .push(
                PeakMeter::new(
                    &mut self.peak_meter_state,
                    util::gain_to_db(self.peak_meter.load(Ordering::Relaxed)),
                )
                    .hold_time(Duration::from_millis(600))
            )
            .into()
    }

    fn background_color(&self) -> Color {
        Color::from_rgb8(0x14, 0x14, 0x14)
    }
}