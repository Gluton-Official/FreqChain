use crate::freqchain::FreqChainParams;
use crate::modules::equalizer::BandType;
use crate::ui::theme::FreqChainTheme;
use crate::ui::theme::Theme;
use crate::ui::themeable::Themeable;
use crate::ui::widgets::equalizer::Equalizer;
use crate::ui::widgets::group::Group;
use crate::ui::widgets::param_knob::{Anchor, ParamKnob};
use crate::ui::widgets::param_slider::ParamSlider;
use crate::ui::widgets::param_toggle::ParamToggle;
use crate::ui::widgets::spectrum::Spectrum;
use crate::ui::widgets::{equalizer, param_knob, param_slider, spectrum};
use crate::ui::{ColorUtils, PaddingExt};
use crate::FreqChain;
use atomic_refcell::AtomicRefCell;
use nih_plug::prelude::*;
use nih_plug_iced::widgets::ParamMessage;
use nih_plug_iced::{alignment, assets, create_iced_editor, executor, Alignment, Color, Column, Command, Container, Element, IcedEditor, IcedState, Length, Padding, Row, Rule, Space, Text, WindowQueue};
use realfft::num_complex::Complex32;
use std::sync::atomic::Ordering;
use std::sync::Arc;

const EDITOR_WIDTH: u32 = 1000;
const EDITOR_HEIGHT: u32 = 572;

pub(crate) fn default_state() -> Arc<IcedState> {
    IcedState::from_size(EDITOR_WIDTH, EDITOR_HEIGHT)
}

pub(crate) fn create<const CHANNELS: usize, const WINDOW_SIZE: usize>(
    params: Arc<FreqChainParams>,
    sample_rate: Arc<AtomicF32>,
    input_buffer_out: Arc<AtomicRefCell<triple_buffer::Output<[[Complex32; WINDOW_SIZE]; CHANNELS]>>>,
    sidechain_buffer_out: Arc<AtomicRefCell<triple_buffer::Output<[[Complex32; WINDOW_SIZE]; CHANNELS]>>>,
    output_buffer_out: Arc<AtomicRefCell<triple_buffer::Output<[[Complex32; WINDOW_SIZE]; CHANNELS]>>>,
    editor_state: Arc<IcedState>,
) -> Option<Box<dyn Editor>> {
    create_iced_editor::<FreqChainEditor<CHANNELS, WINDOW_SIZE>>(
        editor_state, 
        (params, sample_rate, input_buffer_out, sidechain_buffer_out, output_buffer_out)
    )
}

pub struct FreqChainEditor<const CHANNELS: usize, const WINDOW_SIZE: usize> {
    params: Arc<FreqChainParams>,
    sample_rate: Arc<AtomicF32>,

    theme: Theme,

    context: Arc<dyn GuiContext>,

    sidechain_gain_state: param_slider::State,
    sidechain_detail_state: param_knob::State,
    sidechain_precision_state: param_knob::State,

    smoother_attack_state: param_knob::State,
    smoother_decay_state: param_knob::State,

    band_gain_state: param_slider::State,
    band_frequency_state: param_knob::State,
    band_q_state: param_knob::State,
    
    input_spectrum_state: spectrum::State<CHANNELS, WINDOW_SIZE>,
    sidechain_spectrum_state: spectrum::State<CHANNELS, WINDOW_SIZE>,
    output_spectrum_state: spectrum::State<CHANNELS, WINDOW_SIZE>,

    equalizer_state: equalizer::State,
}

/// Messages to be sent to the editor UI
#[derive(Debug, Clone, Copy)]
pub enum Message {
    ParamUpdate(ParamMessage),
}

impl<const CHANNELS: usize, const WINDOW_SIZE: usize> IcedEditor for FreqChainEditor<CHANNELS, WINDOW_SIZE> {
    type Executor = executor::Default;
    type Message = Message;
    type InitializationFlags = (
        Arc<FreqChainParams>,
        Arc<AtomicF32>, // sample rate
        Arc<AtomicRefCell<triple_buffer::Output<[[Complex32; WINDOW_SIZE]; CHANNELS]>>>, // input buffer out
        Arc<AtomicRefCell<triple_buffer::Output<[[Complex32; WINDOW_SIZE]; CHANNELS]>>>, // sidechain buffer out
        Arc<AtomicRefCell<triple_buffer::Output<[[Complex32; WINDOW_SIZE]; CHANNELS]>>>, // output buffer out
    );

    fn new(
        (params, sample_rate, input_buffer_out, sidechain_buffer_out, output_buffer_out): Self::InitializationFlags,
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

            smoother_attack_state: param_knob::State::default(),
            smoother_decay_state: param_knob::State::default(),

            band_gain_state: param_slider::State::default(),
            band_frequency_state: param_knob::State::default(),
            band_q_state: param_knob::State::default(),
            
            input_spectrum_state: spectrum::State::<CHANNELS, WINDOW_SIZE>::new(input_buffer_out),
            sidechain_spectrum_state: spectrum::State::<CHANNELS, WINDOW_SIZE>::new(sidechain_buffer_out),
            output_spectrum_state: spectrum::State::<CHANNELS, WINDOW_SIZE>::new(output_buffer_out),

            equalizer_state: equalizer::State::default(),
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
        // NOTE: the layout size given to elements has already been scaled,
        //       meaning hard-set sizes don't scale, even though elements can see they have more room
        
        let vertical_label = |text: &str| {
            Column::<Self::Message>::with_children(
                text.chars().map(|char| {
                    Text::new(char)
                        .apply_theme(self.theme)
                        .color(self.theme.foreground.with_alpha(0.5))
                        .height(10.into())
                        .into()
                }).collect()
            )
                .align_items(Alignment::Center)
        };

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
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .spacing(2)
            .push(
                Row::new()
                    .align_items(Alignment::Fill)
                    .push(author)
                    .push(version)
            )
            .push(title);

        let frequency_sidechain_label = vertical_label("FREQUENCY_SIDECHAIN")
            .padding(Padding::right(14));

        let sidechain_gain = ParamSlider::new(
            &mut self.sidechain_gain_state,
            &self.params.sidechain_input.gain
        )
            .label("Gain")
            .style(self.theme.slider(
                FloatRange::Skewed {
                    min: util::db_to_gain(util::MINUS_INFINITY_DB),
                    max: util::db_to_gain(24.0),
                    factor: FloatRange::gain_skew_factor(util::MINUS_INFINITY_DB, 24.0)
                }
                    .normalize(util::db_to_gain(0.0))
            ))
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
            .anchor(Anchor::Center)
            .map(Message::ParamUpdate);

        let frequency_sidechain = Row::new()
            .width(Length::Fill)
            .max_height(190)
            .align_items(Alignment::Center)
            .push(frequency_sidechain_label)
            .push(sidechain_gain)
            .push(Row::new()
                .padding(Padding::left(14))
                .align_items(Alignment::Center)
                .width(Length::FillPortion(1))
                .height(Length::Fill)
                .push(Column::with_children(vec![mono_toggle, stereo_toggle]))
            )
            .push(Column::new()
                .align_items(Alignment::Fill)
                .width(Length::FillPortion(3))
                .height(Length::Fill)
                .spacing(4)
                .push(detail)
                .push(precision)
            );

        let smoother_label: Element<Message> = Element::<Message>::from(vertical_label("SMOOTHER")
            .padding(Padding::right(8)))
            // .explain(Color::from_rgb8(0, 255, 255))
            ;

        let attack_label = Element::<Message>::from(Text::new("Attack")
            .apply_theme(self.theme)
            .width(Length::Fill)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center))
            // .explain(Color::from_rgb8(255, 0, 255))
            ;
        let attack_bypass = ParamToggle::new(
            &self.params.frequency_sidechain.smoother.attack_bypass
        )
            .associated_value(false)
            .style(self.theme.radio_toggle(None))
            .width(10.into())
            .height(10.into())
            .map(Message::ParamUpdate)
            // .explain(Color::from_rgb8(255, 0, 0))
            ;

        let attack_header = Row::new()
            .align_items(Alignment::Center)
            .push(Space::with_width(10.into()))
            .push(attack_label)
            .push(attack_bypass);

        let attack_knob = ParamKnob::new(
            &mut self.smoother_attack_state,
            &self.params.frequency_sidechain.smoother.attack_speed
        )
            .label("Speed")
            .apply_theme(self.theme)
            .width(Length::Fill)
            .height(Length::Fill)
            .map(Message::ParamUpdate)
            // .explain(Color::from_rgb8(0, 255, 0))
            ;

        let decay_label = Element::<Message>::from(Text::new("Decay")
            .apply_theme(self.theme)
            .width(Length::Fill)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center))
            // .explain(Color::from_rgb8(255, 0, 255))
            ;
        let decay_bypass = ParamToggle::new(
            &self.params.frequency_sidechain.smoother.decay_bypass
        )
            .associated_value(false)
            .style(self.theme.radio_toggle(None))
            .width(10.into())
            .height(10.into())
            .map(Message::ParamUpdate)
            // .explain(Color::from_rgb8(0, 0, 255))
            ;

        let decay_header = Row::new()
            .align_items(Alignment::Center)
            .push(Space::with_width(10.into()))
            .push(decay_label)
            .push(decay_bypass);

        let decay_knob = ParamKnob::new(
            &mut self.smoother_decay_state,
            &self.params.frequency_sidechain.smoother.decay_speed
        )
            .label("Speed")
            .apply_theme(self.theme)
            .width(Length::Fill)
            .height(Length::Fill)
            .map(Message::ParamUpdate)
            // .explain(Color::from_rgb8(255, 255, 0))
            ;

        let smoother = Row::new()
            .width(Length::Fill)
            .max_height(90)
            .align_items(Alignment::Center)
            .push(smoother_label)
            .push(Row::new()
                .width(Length::Fill)
                .height(Length::Fill)
                .spacing(8)
                .push(Column::new()
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .spacing(4)
                    .push(attack_header)
                    .push(attack_knob)
                )
                .push(Rule::vertical(1).apply_theme(self.theme))
                .push(Column::new()
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .spacing(4)
                    .push(decay_header)
                    .push(decay_knob)
                )
            );

        let equalizer_label = vertical_label("EQUALIZER")
            .padding(Padding::right(14));

        let band_colors = <Theme as equalizer::StyleSheet>::style(&self.theme).band_colors;
        let active_node_index = self.equalizer_state.active_node_index();
        let active_band = &self.params.equalizer.bands[active_node_index];
        let active_band_color = band_colors[active_node_index];

        let band_label = Element::<Message>::from(Text::new(format!("Band {}", active_node_index + 1))
            .apply_theme(self.theme)
            .width(Length::Fill)
            .color(band_colors[active_node_index])
            .horizontal_alignment(alignment::Horizontal::Left)
            .vertical_alignment(alignment::Vertical::Center))
            // .explain(Color::from_rgb8(255, 0, 0))
            ;
        let band_bypass = ParamToggle::new(
            &active_band.bypass
        )
            .associated_value(false)
            .style(self.theme.radio_toggle(Some(band_colors[active_node_index])))
            .width(10.into())
            .height(10.into())
            .map(Message::ParamUpdate)
            // .explain(Color::from_rgb8(0, 255, 0))
            ;

        let band_header = Element::<Message>::from(Row::new()
            .align_items(Alignment::Center)
            .push(Space::with_width(32.into()))
            .push(band_label)
            .push(band_bypass))
            // .explain(Color::from_rgb8(0, 0, 255))
            ;

        let band_gain = ParamSlider::new(
            &mut self.band_gain_state,
            &active_band.gain
        )
            .label("Gain")
            .style(self.theme.slider(0.5))
            .width(26.into())
            .height(Length::Fill)
            .map(Message::ParamUpdate)
            // .explain(Color::from_rgb8(255, 0, 255))
            ;
        let band_frequency = ParamKnob::new(
            &mut self.band_frequency_state,
            &active_band.frequency
        )
            .label("Frequency")
            .apply_theme(self.theme)
            .width(Length::Fill)
            .height(Length::Fill)
            .map(Message::ParamUpdate)
            // .explain(Color::from_rgb8(0, 255, 255))
            ;
        let band_q = ParamKnob::new(
            &mut self.band_q_state,
            &active_band.q
        )
            .label("Q")
            .anchor(Anchor::Center)
            .apply_theme(self.theme)
            .width(Length::Fill)
            .height(Length::Fill)
            .map(Message::ParamUpdate)
            // .explain(Color::from_rgb8(255, 255, 0))
            ;

        let low_shelf_toggle = ParamToggle::new(
            &active_band.band_type,
        )
            .associated_value_exclusive(BandType::LowShelf)
            .style(self.theme.band_shape_toggle(BandType::LowShelf, active_band_color))
            .width(Length::FillPortion(3))
            .height(Length::Fill)
            .map(Message::ParamUpdate);
        let peak_toggle = ParamToggle::new(
            &active_band.band_type,
        )
            .associated_value_exclusive(BandType::Peak)
            .style(self.theme.band_shape_toggle(BandType::Peak, active_band_color))
            .width(Length::FillPortion(2))
            .height(Length::Fill)
            .map(Message::ParamUpdate);
        let high_shelf_toggle = ParamToggle::new(
            &active_band.band_type,
        )
            .associated_value_exclusive(BandType::HighShelf)
            .style(self.theme.band_shape_toggle(BandType::HighShelf, active_band_color))
            .width(Length::FillPortion(3))
            .height(Length::Fill)
            .map(Message::ParamUpdate);

        let high_pass_toggle = ParamToggle::new(
            &active_band.band_type,
        )
            .associated_value_exclusive(BandType::HighPass)
            .style(self.theme.band_shape_toggle(BandType::HighPass, active_band_color))
            .width(Length::FillPortion(3))
            .height(Length::Fill)
            .map(Message::ParamUpdate);
        let notch_toggle = ParamToggle::new(
            &active_band.band_type,
        )
            .associated_value_exclusive(BandType::Notch)
            .style(self.theme.band_shape_toggle(BandType::Notch, active_band_color))
            .width(Length::FillPortion(2))
            .height(Length::Fill)
            .map(Message::ParamUpdate);
        let low_pass_toggle = ParamToggle::new(
            &active_band.band_type,
        )
            .associated_value_exclusive(BandType::LowPass)
            .style(self.theme.band_shape_toggle(BandType::LowPass, active_band_color))
            .width(Length::FillPortion(3))
            .height(Length::Fill)
            .map(Message::ParamUpdate);

        let equalizer_group = Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Alignment::Center)
            .push(equalizer_label)
            .push(band_gain)
            .push(Space::with_width(8.into()))
            .push(Column::new()
                .width(Length::Fill)
                .height(Length::Fill)
                .push(band_header)
                .push(Space::with_height(4.into()))
                .push(Row::new()
                    .width(Length::Fill)
                    .height(Length::FillPortion(3))
                    .spacing(8)
                    .push(band_frequency)
                    .push(band_q)
                )
                .push(Space::with_height(8.into()))
                .push(Element::<Message>::from(Column::new()
                    .width(Length::Fill)
                    .height(Length::FillPortion(2))
                    .padding(8)
                    .spacing(12)
                    .push(Row::new()
                        .width(Length::Fill)
                        .height(Length::FillPortion(1))
                        .spacing(12)
                        .push(low_shelf_toggle)
                        .push(peak_toggle)
                        .push(high_shelf_toggle)
                    )
                    .push(Row::new()
                        .width(Length::Fill)
                        .height(Length::FillPortion(1))
                        .spacing(12)
                        .push(high_pass_toggle)
                        .push(notch_toggle)
                        .push(low_pass_toggle)
                    ))
                    // .explain(Color::from_rgb8(255, 0, 255))
                )
            );
        
        let input_spectrum = Spectrum::new(
            &mut self.input_spectrum_state,
        )
            .style(self.theme.spectrum(Color::from_rgb8(126, 126, 126), spectrum::Alignment::Bottom))
            .width(Length::Fill)
            .height(Length::Fill);
        let output_spectrum = Spectrum::new(
            &mut self.output_spectrum_state,
        )
            .style(self.theme.spectrum(Color::from_rgb8(58, 173, 255), spectrum::Alignment::Bottom))
            .width(Length::Fill)
            .height(Length::Fill);
        
        let sidechain_spectrum = Spectrum::new(
            &mut self.sidechain_spectrum_state
        )
            .style(self.theme.spectrum(Color::from_rgb8(255, 50, 50), spectrum::Alignment::Bottom))
            .width(Length::Fill)
            .height(Length::Fill);

        let equalizer = Equalizer::new(
            &mut self.equalizer_state,
            &self.params.equalizer,
            self.sample_rate.load(Ordering::Relaxed),
        )
            .apply_theme(self.theme)
            .width(Length::Fill)
            .height(Length::Fill)
            .map(Message::ParamUpdate);
        
        let eq_enabled = ParamToggle::new(
            &self.params.equalizer.enabled
        )
            .style(self.theme.power_button())
            .width(12.into())
            .height(12.into())
            .map(Message::ParamUpdate);
        let sidechain_solo = ParamToggle::new(
            &self.params.sidechain_input.solo
        )
            .style(self.theme.headphones_button())
            .width(12.into())
            .height(12.into())
            .map(Message::ParamUpdate);

        Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(Column::new()
                .align_items(Alignment::Center)
                .width(192.into())
                .padding(8)
                .push(header)
                .push(Space::with_height(12.into()))
                .push(Rule::horizontal(1).apply_theme(self.theme))
                .push(Space::with_height(12.into()))
                .push(frequency_sidechain)
                .push(Space::with_height(12.into()))
                .push(Rule::horizontal(1).apply_theme(self.theme))
                .push(Space::with_height(8.into()))
                .push(smoother)
                .push(Space::with_height(12.into()))
                .push(Rule::horizontal(1).apply_theme(self.theme))
                .push(Space::with_height(8.into()))
                .push(equalizer_group)
            )
            .push(Column::new()
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(Padding { top: 4, right: 4, bottom: 4, left: 0 })
                .spacing(4)
                .push(Container::new(
                    Group::new()
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .push(input_spectrum)
                        .push(output_spectrum)
                )
                    .apply_theme(self.theme)
                    .width(Length::Fill)
                    .height(80.into())
                    .align_x(alignment::Horizontal::Center)
                    .align_y(alignment::Vertical::Center)
                    .padding(1)
                )
                .push(Container::new(
                    Group::new()
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .push(sidechain_spectrum)
                        .push(equalizer)
                        .push(Row::with_children(vec![eq_enabled, sidechain_solo])
                            .padding(6)
                            .spacing(6)
                        )
                )
                    .apply_theme(self.theme)
                    .width(Length::Fill)
                    .height(480.into())
                    .max_height(480)
                    .padding(1)
                )
            )
            .into()
    }

    fn background_color(&self) -> Color {
        self.theme.background
    }
}