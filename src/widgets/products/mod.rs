use crate::messages::Message;
use fltk::{
    app::{redraw, Sender},
    button::Button,
    draw::set_cursor,
    enums::{Align, Color, Cursor, Event, FrameType},
    prelude::*,
    widget_extends,
};
widget_extends!(ProductFrame, Button, pd_frame);

#[derive(Clone, Debug)]
pub struct ProductFrame {
    pd_frame: Button,
}

impl ProductFrame {
    pub fn new(x: i32, y: i32, w: i32, h: i32, title: &str, tx: Sender<Message>) -> Self {
        let label;
        if title.len() > 30 {
            label = title[0..=30].to_string() + "...";
        } else {
            label = title.to_string();
        }
        let mut pd_frame = Button::new(x, y, w, h, None).with_label(&label);
        pd_frame.clear_visible_focus();
        pd_frame.set_frame(FrameType::FlatBox);
        pd_frame.set_color(Color::DarkYellow);
        pd_frame.set_selection_color(Color::DarkYellow);
        pd_frame.set_align(Align::Center);
        pd_frame.set_label_color(Color::Black);
        pd_frame.set_label_size(10);
        pd_frame.handle(move |b, ev| match ev {
            Event::Enter => {
                set_cursor(Cursor::Hand);
                b.set_color(Color::Cyan);
                redraw();
                true
            }
            Event::Leave => {
                set_cursor(Cursor::Default);
                b.set_color(Color::DarkYellow);
                redraw();
                true
            }
            Event::Push => {
                b.emit(tx.clone(), Message::ProductModify(b.clone()));
                set_cursor(Cursor::Hand);
                redraw();
                true
            }
            Event::Focus => {
                set_cursor(Cursor::Hand);
                redraw();
                true
            }
            _ => false,
        });
        Self { pd_frame }
    }
}