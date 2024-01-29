use std::{
    num::NonZeroU32,
    sync::{atomic::Ordering, Arc, Mutex},
};

use nih_plug::prelude::*;

use crate::{
    audio_processing::{
        frequency_sidechain::FrequencySidechain,
        spectrum::{Spectrum, SpectrumOutput},
    },
    params::FreqChainParams,
    ui::editor,
};

const SMOOTHING_DECAY_MS: f32 = 150.0;

const CHANNELS: usize = 2;

pub struct FreqChain {
    params: Arc<FreqChainParams>,

    sample_rate: Arc<AtomicF32>,

    /// The weight to apply to the previous peak values when calculating the new peak value,
    /// Usable for peak meter, spectrum, etc.
    smoothing_decay_weight: f32,

    input_peak_meter_value: Arc<AtomicF32>,
    output_peak_meter_value: Arc<AtomicF32>,
    sidechain_input_peak_meter_value: Arc<AtomicF32>,
    sidechain_output_peak_meter_value: Arc<AtomicF32>,

    sidechain_spectrum: Spectrum,
    sidechain_spectrum_output: Arc<Mutex<SpectrumOutput>>,

    frequency_sidechain: FrequencySidechain,
}

impl Default for FreqChain {
    fn default() -> Self {
        let (sidechain_spectrum, sidechain_spectrum_output) = Spectrum::new();

        Self {
            params: Arc::new(FreqChainParams::default()),

            sample_rate: Arc::new(AtomicF32::new(1.0)),

            smoothing_decay_weight: 1.0,

            input_peak_meter_value: Arc::new(AtomicF32::new(util::MINUS_INFINITY_DB)),
            output_peak_meter_value: Arc::new(AtomicF32::new(util::MINUS_INFINITY_DB)),
            sidechain_input_peak_meter_value: Arc::new(AtomicF32::new(util::MINUS_INFINITY_DB)),
            sidechain_output_peak_meter_value: Arc::new(AtomicF32::new(util::MINUS_INFINITY_DB)),

            sidechain_spectrum,
            sidechain_spectrum_output: Arc::new(Mutex::new(sidechain_spectrum_output)),

            frequency_sidechain: FrequencySidechain::new(),
        }
    }
}

impl Plugin for FreqChain {
    const NAME: &'static str = "FreqChain";
    const VENDOR: &'static str = "Gluton";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "glutonofficial@gmail.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        // Stereo layout
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(CHANNELS as u32),
            main_output_channels: NonZeroU32::new(CHANNELS as u32),

            aux_input_ports: &[new_nonzero_u32(CHANNELS as u32)],

            ..AudioIOLayout::const_default()
        },
    ];

    /// Tell the wrapper to split the audio buffer into smaller blocks when there are inter-buffer
    /// parameter changes, allowing the wrapper to handle transport and timing information between
    /// splits.
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    // Do not support sending/receiving SysEx MIDI messages
    type SysExMessage = ();
    // No background tasks
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(
            self.params.clone(),
            self.sample_rate.clone(),
            self.input_peak_meter_value.clone(),
            self.sidechain_spectrum_output.clone(),
            self.params.editor_state.clone(),
        )
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        context: &mut impl InitContext<Self>,
    ) -> bool {
        self.sample_rate = Arc::new(AtomicF32::new(buffer_config.sample_rate));

        // After `SMOOTHING_DECAY_MS` milliseconds of silence, the peak meter's value should drop by 12 dB
        self.smoothing_decay_weight = 0.25f32.powf((buffer_config.sample_rate * SMOOTHING_DECAY_MS / 1000.0).recip());

        self.sidechain_spectrum
            .set_smoothing_decay_weight(self.smoothing_decay_weight);

        context.set_latency_samples(self.frequency_sidechain.latency_samples());

        true
    }

    fn reset(&mut self) {
        self.frequency_sidechain.reset()
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let sidechain_buffer = &mut aux.inputs[0];

        nih_debug_assert_eq!(
            buffer.channels(),
            sidechain_buffer.channels(),
            "Main and sidechain buffer channels differ"
        );
        nih_debug_assert_eq!(
            buffer.samples(),
            sidechain_buffer.samples(),
            "Main and sidechain buffer samples differ"
        );

        self.frequency_sidechain.ensure_channels(self.channels());
        self.frequency_sidechain.process(buffer, sidechain_buffer);

        ProcessStatus::Normal // allow for suspense if no input audio
    }
}

impl FreqChain {
    fn update_peak_meter(&self, peak_meter_value: &Arc<AtomicF32>, amplitude: f32) {
        let current_peak_meter_value = peak_meter_value.load(Ordering::Relaxed);
        // Increase the peak meter value or gradually decay it
        let new_peak_meter_value = if amplitude > current_peak_meter_value {
            amplitude
        } else {
            current_peak_meter_value * self.smoothing_decay_weight + amplitude * (1.0 - self.smoothing_decay_weight)
        };

        peak_meter_value.store(new_peak_meter_value, Ordering::Relaxed);
    }

    fn channels(&self) -> usize {
        if self.params.is_mono.value() {
            1
        } else {
            CHANNELS
        }
    }
}
