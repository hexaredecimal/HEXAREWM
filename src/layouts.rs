use penrose::{
    builtin::layout::{
        transformers::{Gaps, ReserveTop},
        MainAndStack, Monocle,
    },
    core::layout::LayoutStack,
    stack,
};

const MAX_MAIN: u32 = 1;
const RATIO: f32 = 0.6;
const RATIO_STEP: f32 = 0.1;
const OUTER_PX: u32 = 5;
const INNER_PX: u32 = 5;
const BAR_HEIGHT_PX: u32 = 18;

pub struct WmLayout;
impl WmLayout {
    pub fn layouts() -> LayoutStack {
        stack!(
            MainAndStack::boxed_default(),
            MainAndStack::side(MAX_MAIN, RATIO, RATIO_STEP),
            MainAndStack::side_mirrored(MAX_MAIN, RATIO, RATIO_STEP),
            MainAndStack::bottom(MAX_MAIN, RATIO, RATIO_STEP),
            MainAndStack::top(MAX_MAIN, RATIO, RATIO_STEP),
            Monocle::boxed()
        )
        .map(|layout| ReserveTop::wrap(Gaps::wrap(layout, OUTER_PX, INNER_PX), BAR_HEIGHT_PX))
    }
}
