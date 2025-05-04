use std::sync::Arc;
use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;
use parking_lot::RwLock;
use ruadio::{
    buffer_view::BufferViewMut,
    effects::{DigitalDelay, Effect},
};

mod editor;

/// The digital delay plugin
pub struct DigitalDelayPlugin {
    params: Arc<DigitalDelayParams>,
    /// The internal rustafx processor
    processor: Arc<RwLock<DigitalDelay>>,
}

#[derive(Params)]
struct DigitalDelayParams {
    /// The editor state, saved together with the parameter state so the custom scaling can be
    /// restored.
    #[persist = "editor-state"]
    editor_state: Arc<ViziaState>,

    #[id = "delay"]
    pub delay: FloatParam,
    #[id = "feedback"]
    pub feedback: FloatParam,
    #[id = "dry_gain"]
    pub dry_gain: FloatParam,
    #[id = "wet_gain"]
    pub wet_gain: FloatParam,
}

impl Default for DigitalDelayPlugin {
    fn default() -> Self {
        Self {
            params: Arc::new(DigitalDelayParams::default()),
            processor: Arc::new(RwLock::new(DigitalDelay::new(2))),
        }
    }
}

impl Default for DigitalDelayParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),

            delay: FloatParam::new(
                "Delay",
                50.0,
                FloatRange::Linear { min: 1.0, max: 1000.0 },
            )
                .with_unit(" ms")
                .with_value_to_string(formatters::v2s_f32_rounded(1)),

            feedback: FloatParam::new(
                "Feedback",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
                .with_unit("%")
                .with_value_to_string(formatters::v2s_f32_percentage(1))
                .with_string_to_value(formatters::s2v_f32_percentage()),

            dry_gain: FloatParam::new(
                "Dry Level",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-60.0),
                    max: util::db_to_gain(0.0),
                    factor: FloatRange::gain_skew_factor(-60.0, 0.0),
                },
            )
                .with_unit(" dB")
                .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
                .with_string_to_value(formatters::s2v_f32_gain_to_db()),

            wet_gain: FloatParam::new(
                "Wet Level",
                util::db_to_gain(-20.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-60.0),
                    max: util::db_to_gain(0.0),
                    factor: FloatRange::gain_skew_factor(-60.0, 0.0),
                },
            )
                .with_unit(" dB")
                .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
                .with_string_to_value(formatters::s2v_f32_gain_to_db()),
        }
    }
}

impl Plugin for DigitalDelayPlugin {
    const NAME: &'static str = "Digital Delay";
    const VENDOR: &'static str = "HSW Audio";
    const URL: &'static str = "https://github.com/hswangTW/rustafx-plugins";
    const EMAIL: &'static str = "hanson2693@gmail.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(self.params.clone(), self.params.editor_state.clone())
    }

    fn initialize(
        &mut self,
        audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // XXX Consider making effect processors reusable for possible channel layout changes
        let num_channels = audio_io_layout.main_input_channels.unwrap().get() as usize;
        self.processor = Arc::new(RwLock::new(DigitalDelay::new(num_channels)));

        // Initialize the processor
        let mut processor = self.processor.write();
        processor.set_delay_time(self.params.delay.value());
        processor.set_feedback(self.params.feedback.value());
        processor.set_dry_gain(self.params.dry_gain.value());
        processor.set_wet_gain(self.params.wet_gain.value());

        processor.prepare(buffer_config.sample_rate, buffer_config.max_buffer_size as usize);

        true
    }

    fn reset(&mut self) {
        let mut processor = self.processor.write();
        processor.reset();
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let mut processor = self.processor.write();

        // Update the parameters
        processor.set_delay_time(self.params.delay.value());
        processor.set_feedback(self.params.feedback.value());
        processor.set_dry_gain(self.params.dry_gain.value());
        processor.set_wet_gain(self.params.wet_gain.value());

        // Process the buffer
        let slice = unsafe { std::mem::transmute(buffer.as_slice()) };
        let mut buffer_view = BufferViewMut::new(slice);
        processor.process_inplace(&mut buffer_view);

        ProcessStatus::Normal
    }
}

impl Vst3Plugin for DigitalDelayPlugin {
    const VST3_CLASS_ID: [u8; 16] = *b"RustafxDigiDelay";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Delay];
}

nih_export_vst3!(DigitalDelayPlugin);
