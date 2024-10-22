use std::sync::{Arc, Mutex};

use nih_plug::prelude::*;
use nih_plug_iced::{
    alignment, assets, create_iced_editor, executor, text_input,
    widgets::{param_slider, peak_meter, ParamMessage, ParamSlider},
    Alignment, Color, Column, Command, Element, IcedEditor, IcedState, Length, Row, Text, WindowQueue,
};

use crate::{
    modules::spectrum::SpectrumOutput,
    ui::widgets::param_knob::{self},
    FreqChain,
};
use crate::freqchain::FreqChainParams;

const EDITOR_WIDTH: u32 = 896;
const EDITOR_HEIGHT: u32 = 512;

pub(crate) fn default_state() -> Arc<IcedState> {
    IcedState::from_size(EDITOR_WIDTH, EDITOR_HEIGHT)
}

pub(crate) fn create(
    params: Arc<FreqChainParams>,
    sample_rate: Arc<AtomicF32>,
    peak_meter: Arc<AtomicF32>,
    sidechain_spectrum: Arc<Mutex<SpectrumOutput>>,
    editor_state: Arc<IcedState>,
) -> Option<Box<dyn Editor>> {
    create_iced_editor::<FreqChainEditor>(editor_state, (params, sample_rate, peak_meter, sidechain_spectrum))
}

pub struct FreqChainEditor {
    params: Arc<FreqChainParams>,
    context: Arc<dyn GuiContext>,

    sample_rate: Arc<AtomicF32>,

    peak_meter: Arc<AtomicF32>,
    sidechain_spectrum: Arc<Mutex<SpectrumOutput>>,

    sidechain_gain_slider_state: param_slider::State,
    sidechain_gain_input_state: text_input::State,
    sidechain_detail_knob_state: param_knob::State,
    sidechain_detail_input_state: text_input::State,
    peak_meter_state: peak_meter::State,
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
        Arc<AtomicF32>,             // peak meter
        Arc<Mutex<SpectrumOutput>>, // sidechain spectrum
    );

    fn new(
        (params, sample_rate, peak_meter, sidechain_spectrum): Self::InitializationFlags,
        context: Arc<dyn GuiContext>,
    ) -> (Self, Command<Self::Message>) {
        let editor = FreqChainEditor {
            params,
            context,

            sample_rate,

            peak_meter,
            sidechain_spectrum,

            sidechain_gain_slider_state: Default::default(),
            sidechain_gain_input_state: Default::default(),
            sidechain_detail_knob_state: Default::default(),
            sidechain_detail_input_state: Default::default(),
            peak_meter_state: Default::default(),
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
            .font(assets::NOTO_SANS_LIGHT)
            .size(12)
            .height(20.into())
            .width(Length::FillPortion(1))
            .horizontal_alignment(alignment::Horizontal::Left)
            .vertical_alignment(alignment::Vertical::Center);
        let version = Text::new(FreqChain::VERSION)
            .font(assets::NOTO_SANS_LIGHT)
            .size(12)
            .height(20.into())
            .width(Length::FillPortion(1))
            .horizontal_alignment(alignment::Horizontal::Right)
            .vertical_alignment(alignment::Vertical::Center);
        let _author_version = Row::<Message>::new()
            .align_items(Alignment::Fill)
            .push(author)
            .push(version);

        let _title = Text::new(FreqChain::NAME)
            .font(assets::NOTO_SANS_LIGHT)
            .size(24)
            .height(50.into())
            .width(Length::Fill)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center);

        // let sidechain_tab = Rectangle::new(Point::ORIGIN, Size::new(40.0, 20.0))
        //     .align_items(Alignment::Center)
        //     .push(
        //         Text::new("Sidechain")
        //             .height(20.into())
        //             .width(Length::Fill)
        //             .horizontal_alignment(alignment::Horizontal::Center)
        //             .vertical_alignment(alignment::Vertical::Center)
        //     );
        // let eq_tab = Rectangle::new(Point::ORIGIN, Size::new(40.0, 20.0))
        //     .align_items(Alignment::Center)
        //     .push(
        //         Text::new("EQ")
        //             .height(20.into())
        //             .width(Length::Fill)
        //             .horizontal_alignment(alignment::Horizontal::Center)
        //             .vertical_alignment(alignment::Vertical::Center)
        //     );
        // let tabs = Row::new()
        //     .align_items(Alignment::Center)
        //     .push(sidechain_tab)
        //     .push(eq_tab);

        let sidechain_gain_slider =
            ParamSlider::new(&mut self.sidechain_gain_slider_state, &self.params.sidechain_input.gain)
                .map(Message::ParamUpdate);
        // let sidechain_gain_input = TextInput::new(
        //     &mut self.sidechain_gain_input_state,
        //     "0db",
        //     &format!("{}", util::gain_to_db(self.params.gain.smoothed.next())),
        //     Message::ParamUpdate
        // );
        let sidechain_gain_label = Text::new("Gain")
            .height(20.into())
            .width(Length::Fill)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center);
        let sidechain_gain = Column::new()
            .align_items(Alignment::Center)
            // .push(sidechain_gain_input)
            .push(sidechain_gain_slider)
            .push(sidechain_gain_label);

        // let sidechain_detail_knob =
        //     ParamKnob::new(&mut self.sidechain_detail_knob_state, &self.params.detail).map(Message::ParamUpdate);
        // let sidechain_detail_input = TextInput::new(
        //     &mut self.sidechain_detail_input_state,
        //     "0%",
        //     &format!("{}", self.params.detail),
        //     Message::ParamUpdate
        // );
        // let sidechain_detail_label = Text::new("Detail")
        //     .height(20.into())
        //     .width(Length::Fill)
        //     .horizontal_alignment(alignment::Horizontal::Center)
        //     .vertical_alignment(alignment::Vertical::Center);
        // let sidechain_detail = Column::new()
        //     .align_items(Alignment::Center)
            // .push(sidechain_detail_input)
            // .push(sidechain_detail_knob)
            // .push(sidechain_detail_label);

        // let sidechain_spectrum = Spectrum::new(
        //     self.sidechain_spectrum.clone(),
        //     self.sample_rate.clone(),
        //     spectrum::Normalization::Logarithmic,
        // );

        Column::new()
            .align_items(Alignment::Center)
            .push(Row::new().align_items(Alignment::Center).push(sidechain_gain))
            // .push(Row::new().align_items(Alignment::Center).push(sidechain_detail))
            .into()

        // .push(
        //     PeakMeter::new(
        //         &mut self.peak_meter_state,
        //         util::gain_to_db(self.peak_meter.load(Ordering::Relaxed)),
        //     )
        //         .hold_time(Duration::from_millis(600))
        // )
    }

    fn background_color(&self) -> Color {
        Color::WHITE
    }
}
