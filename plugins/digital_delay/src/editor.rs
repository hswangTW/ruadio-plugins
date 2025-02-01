use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;

use crate::DigitalDelayParams;

#[derive(Lens)]
struct Data {
    params: Arc<DigitalDelayParams>,
}

impl Model for Data {}

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (400, 300))
}

pub(crate) fn create(
    params: Arc<DigitalDelayParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(
        editor_state,
        ViziaTheming::Custom,
        move |cx, _| {
            assets::register_noto_sans_light(cx);
            assets::register_noto_sans_thin(cx);

            Data { params: params.clone() }.build(cx);

            VStack::new(cx, |cx| {
                Label::new(cx, "Digital Delay")
                    .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                    .font_weight(FontWeightKeyword::Thin)
                    .font_size(30.0)
                    .height(Pixels(50.0))
                    .child_top(Stretch(1.0))
                    .child_bottom(Stretch(0.0));

                Label::new(cx, "Delay");
                ParamSlider::new(cx, Data::params, |params| &params.delay);

                Label::new(cx, "Feedback");
                ParamSlider::new(cx, Data::params, |params| &params.feedback);

                Label::new(cx, "Dry level");
                ParamSlider::new(cx, Data::params, |params| &params.dry_gain);

                Label::new(cx, "Wet level");
                ParamSlider::new(cx, Data::params, |params| &params.wet_gain);
            })
                .row_between(Pixels(0.0))
                .child_left(Stretch(1.0))
                .child_right(Stretch(1.0));

            ResizeHandle::new(cx);
        }
    )
}
