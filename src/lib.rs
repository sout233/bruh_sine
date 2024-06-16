use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;
use std::sync::Arc;

mod editor;

// This is a shortened version of the gain example with most comments removed, check out
// https://github.com/robbert-vdh/nih-plug/blob/master/plugins/examples/gain/src/lib.rs to get
// started

pub struct BruhSine {
    params: Arc<BruhSineParams>,
    x: f32,
}

#[derive(Params)]
struct BruhSineParams {
    #[persist = "editor-state"]
    editor_state: Arc<ViziaState>,

    #[id = "increment"]
    pub increment: FloatParam,

    #[id = "factor"]
    pub factor: FloatParam,

    #[id = "mix"]
    pub mix: FloatParam,

    #[id = "output"]
    pub output: FloatParam,
}

impl Default for BruhSine {
    fn default() -> Self {
        Self {
            params: Arc::new(BruhSineParams::default()),
            x: 0.0,
        }
    }
}

impl Default for BruhSineParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),

            increment: FloatParam::new(
                "Increment",
                1.0,
                FloatRange::Skewed {
                    min: 0.01,
                    max: 1.0,
                    factor: 0.5,
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_value_to_string(formatters::v2s_f32_rounded(2)),

            factor: FloatParam::new(
                "Factor",
                1.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 1.0,
                    factor: 0.5,
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_value_to_string(formatters::v2s_f32_rounded(2)),

            mix: FloatParam::new(
                "Mix",
                100.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 100.0,
                    factor: 0.5,
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" %")
            .with_value_to_string(formatters::v2s_f32_rounded(2)),

            output: FloatParam::new(
                "Output",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
        }
    }
}

impl Plugin for BruhSine {
    const NAME: &'static str = "Bruh Sine";
    const VENDOR: &'static str = "sout";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "sout_Nantang@outlook.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    // The first audio IO layout is used as the default. The other layouts may be selected either
    // explicitly or automatically by the host or the user depending on the plugin API/backend.
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),

        aux_input_ports: &[],
        aux_output_ports: &[],

        // Individual ports and the layout as a whole can be named here. By default these names
        // are generated as needed. This layout will be called 'Stereo', while a layout with
        // only one input and output channel would be called 'Mono'.
        names: PortNames::const_default(),
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    // If the plugin can send or receive SysEx messages, it can define a type to wrap around those
    // messages here. The type implements the `SysExMessage` trait, which allows conversion to and
    // from plain byte buffers.
    type SysExMessage = ();
    // More advanced plugins can use this to run expensive background tasks. See the field's
    // documentation for more information. `()` means that the plugin does not have any background
    // tasks.
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // Resize buffers and perform other potentially expensive initialization operations here.
        // The `reset()` function is always called right after this function. You can remove this
        // function if you do not need it.
        true
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(self.params.clone(), self.params.editor_state.clone())
    }

    fn reset(&mut self) {
        // Reset buffers and envelopes here. This can be called from the audio thread and may not
        // allocate. You can remove this function if you do not need it.
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for channel_samples in buffer.iter_samples() {
            let gain = self.params.output.smoothed.next();
            let increment = self.params.increment.smoothed.next();
            let factor = self.params.factor.smoothed.next();
            let mix = self.params.mix.smoothed.next();

            for sample in channel_samples {
                let post_sample = *sample * self.x.powf(factor).sin();
                let mixed_sample = *sample * ((100.0 - mix) / 100.0) + post_sample * (mix / 100.0);
                *sample = mixed_sample * gain;
            }

            self.x += increment;
            if self.x > 10.0 {
                self.x = 0.0;
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for BruhSine {
    const CLAP_ID: &'static str = "org.eu.sout.audio.bruh-sine";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("What the hell with the sine function");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    // Don't forget to change these features
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

impl Vst3Plugin for BruhSine {
    const VST3_CLASS_ID: [u8; 16] = *b"SOUTaudioBruhsin";

    // And also don't forget to change these categories
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Dynamics];
}

nih_export_clap!(BruhSine);
nih_export_vst3!(BruhSine);
