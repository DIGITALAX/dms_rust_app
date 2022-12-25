use fltk::{
    enums::{Align, Color},
    misc::Progress,
    prelude::*,
    widget_extends,
};

widget_extends!(AnimationProgress, Progress, anim_prg);

const BAR_WAIT: f64 = 1.5;

pub struct AnimationProgress {
    anim_prg: Progress,
}

impl AnimationProgress {
    pub fn new() -> Self {
        let mut anim_prg = Progress::new(375, 425, 600, 50, None);
        anim_prg.set_label_color(Color::White);
        anim_prg.set_align(Align::Top);
        anim_prg.set_label_size(15);
        anim_prg.set_maximum(BAR_WAIT);
        Self { anim_prg }
    }
}
