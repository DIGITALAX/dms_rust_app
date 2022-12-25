use fltk::{
    enums::{Color, FrameType, Event, Cursor},
    prelude::*,
    widget_extends,
    button::Button,
    draw::set_cursor,
    app::redraw,
    text::TextDisplay
};

widget_extends!(MenuButton, Button, mbtn);

#[derive(Clone, Debug)]
pub struct MenuButton {
   pub mbtn: Button,
}

impl MenuButton {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str, first_select: bool) -> Self {
        let mut mbtn = Button::new(x, y, w, h, None);
        mbtn.set_frame(FrameType::RoundedBox);
        mbtn.set_selection_color(Color::Cyan);
        mbtn.clear_visible_focus();
        mbtn.set_tooltip(label);
        if first_select {
            mbtn.set_color(Color::Cyan);
        } else {
            mbtn.set_color(Color::DarkYellow);
        }
        mbtn.handle(move |_b, ev| match ev {
            Event::Enter => {
                set_cursor(Cursor::Hand);
                redraw();
                true
            }
            Event::Push => {
                set_cursor(Cursor::Hand);
                redraw();
                true
            }
            Event::Leave => {
                set_cursor(Cursor::Default);
                redraw();
                true
            }
            _ => false,
        });
        let mut title = TextDisplay::new(x + 30, y + 78, 0, 0, None).with_label(label);
        title.set_label_color(Color::White);
        title.set_label_size(10);
        Self { mbtn }
    }
}