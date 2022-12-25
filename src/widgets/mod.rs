use fltk::{
    enums::{Color, Font},
    prelude::*,
    text::TextDisplay,
    widget_extends,
};
pub mod droptypes;
pub mod sidebar;
pub mod animation;
pub mod products;

widget_extends!(MainTitle, TextDisplay, mn_title);

#[derive(Clone)]
pub struct MainTitle {
    mn_title: TextDisplay,
}

impl MainTitle {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str, text_size: i32) -> Self {
        let mut mn_title = TextDisplay::new(x, y, w, h, None).with_label(&label.to_uppercase());
        mn_title.set_label_font(Font::ScreenBold);
        mn_title.set_label_color(Color::White);
        mn_title.set_label_size(text_size);
        Self { mn_title }
    }
}
