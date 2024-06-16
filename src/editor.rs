use self::param_knob::ParamKnob;
use crate::BruhSineParams;
use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;
mod param_knob;

const _SOUT_AUDIO_LOGO: &[u8; 566] = include_bytes!("../res/soutaudio_logo_1x.png");

#[derive(Lens)]
struct Data {
    params: Arc<BruhSineParams>,
}

impl Model for Data {}

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (400, 200))
}

pub(crate) fn create(
    params: Arc<BruhSineParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_bold(cx);
        assets::register_noto_sans_regular(cx);
        cx.add_stylesheet(include_style!("src/editor/style.css"))
            .expect("Failed to load stylesheet");

        // let img = ImageReader::open("res/test.png").unwrap().decode().unwrap();
        // let rgb: RgbImage = RgbImage::new(10, 10);
        // cx.load_image("res/test.png", img, ImageRetentionPolicy::Forever);
        // Image::new(cx, "res/test.png");

        Data {
            params: params.clone(),
        }
        .build(cx);

        // ResizeHandle::new(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, "Bruh Sine")
                .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                .font_weight(FontWeightKeyword::Bold)
                .font_size(30.0)
                .height(Pixels(50.0))
                .child_top(Stretch(1.0))
                .child_bottom(Pixels(0.0))
                .class("title");

            VStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    ParamKnob::new(cx, Data::params, |params| &params.increment, false);
                    ParamKnob::new(cx, Data::params, |params| &params.factor, false);
                    ParamKnob::new(cx, Data::params, |params| &params.mix, false);
                    ParamKnob::new(cx, Data::params, |params| &params.output, false);
                })
                .height(Pixels(100.0))
                .class("knobs");
            })
            .col_between(Pixels(10.0));

            Label::new(cx, "Made with ❤️ by SoutAudio")
                .font_size(12.0)
                .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                .font_weight(FontWeightKeyword::Regular)
                .bottom(Pixels(4.0))
                .class("label");
        })
        .row_between(Pixels(0.0))
        .child_left(Stretch(1.0))
        .child_right(Stretch(1.0))
        .class("root");
    })
}
