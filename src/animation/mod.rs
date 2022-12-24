use crate::widgets::AnimationProgress;
use fltk::{enums::Color, prelude::WidgetExt};

pub(crate) fn start_animation(animation: &mut bool, bar: &mut AnimationProgress) {
    *animation = true;
    bar.set_value(0f64);
    bar.set_color(Color::DarkCyan);
}

pub(crate) fn stop_animation(animation: &mut bool, bar: &mut AnimationProgress) {
    *animation = false;
    bar.set_value(0f64);
    bar.set_color(Color::Green);
}
