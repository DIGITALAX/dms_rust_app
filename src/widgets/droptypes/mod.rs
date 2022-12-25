use crate::{messages::Message, schemas::DropType};
use fltk::{
    app::{redraw, Sender},
    button::Button,
    draw::set_cursor,
    enums::{Align, Color, Cursor, Event, FrameType},
    frame::Frame,
    group::Scroll,
    input::{Input, MultilineInput},
    prelude::*,
    text::TextDisplay,
    widget_extends,
};
use num_integer::div_ceil;
widget_extends!(DropTypeFrame, Button, dt_frame);
widget_extends!(DropTypeInput, Input, dt_input);
widget_extends!(DropTypeMultiInput, MultilineInput, dt_ml_input);
widget_extends!(DropTypeInputLabel, TextDisplay, dt_in_lb);
widget_extends!(DropTypeUnder, Frame, dt_und);
widget_extends!(CRUDButton, Button, crd_btn);
widget_extends!(ReturnButton, Button, r_btn);

#[derive(Clone, Debug)]
pub struct DropTypeFrame {
    dt_frame: Button,
}

impl DropTypeFrame {
    pub fn new(x: i32, y: i32, w: i32, h: i32, title: &str, tx: Sender<Message>) -> Self {
        let label;
        if title.len() > 30 {
            label = title[0..=30].to_string() + "...";
        } else {
            label = title.to_string();
        }
        let mut dt_frame = Button::new(x, y, w, h, None).with_label(&label);
        dt_frame.clear_visible_focus();
        dt_frame.set_frame(FrameType::FlatBox);
        dt_frame.set_color(Color::DarkYellow);
        dt_frame.set_selection_color(Color::DarkYellow);
        dt_frame.set_align(Align::Center);
        dt_frame.set_label_color(Color::Black);
        dt_frame.set_label_size(10);
        dt_frame.handle(move |b, ev| match ev {
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
                b.emit(tx.clone(), Message::DropTypeModify(b.clone()));
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
        Self { dt_frame }
    }
}

pub fn create_droptypes_table(
    droptypes_scroll: &mut Scroll,
    number_of_cols: i32,
    droptypes_len: i32,
    drop_frame_height: i32,
    drop_frame_width: i32,
    row_height: i32,
    col_width: i32,
    mut x_pos: i32,
    mut y_pos: i32,
    droptypes: &Vec<DropType>,
    tx: Sender<Message>,
) {
    let mut title_incramentor = 0;
    for _row in 0..div_ceil(droptypes_len, number_of_cols) as i32 {
        for _col in 0..number_of_cols {
            if title_incramentor < droptypes_len {
                let result = DropTypeFrame::new(
                    x_pos,
                    y_pos,
                    drop_frame_width,
                    drop_frame_height,
                    &droptypes[title_incramentor as usize].title,
                    tx.clone(),
                );
                droptypes_scroll.add(&*result);
                x_pos += col_width;
                title_incramentor += 1;
            } else {
                break;
            }
        }
        x_pos = 200;
        y_pos += row_height;
    }
}

// add drop type
#[derive(Clone, Debug)]
pub struct DropTypeInput {
    dt_input: Input,
}

impl DropTypeInput {
    pub fn new(x: i32, y: i32, w: i32, h: i32, value: Option<&str>) -> Self {
        let mut dt_input = Input::new(x, y, w, h, None);
        dt_input.set_color(Color::BackGround);
        dt_input.set_frame(FrameType::RoundedBox);
        dt_input.set_text_color(Color::White);
        if value != None {
            dt_input.set_value(&value.unwrap());
        }
        Self { dt_input }
    }
}

#[derive(Clone, Debug)]
pub struct DropTypeMultiInput {
    dt_ml_input: MultilineInput,
}

impl DropTypeMultiInput {
    pub fn new(x: i32, y: i32, w: i32, h: i32, value: Option<String>) -> Self {
        let mut dt_ml_input = MultilineInput::new(x, y, w, h, None);
        dt_ml_input.set_color(Color::BackGround);
        dt_ml_input.set_frame(FrameType::RoundedBox);
        dt_ml_input.set_text_color(Color::White);
        if value != None {
            dt_ml_input.set_value(&value.unwrap());
        }
        Self { dt_ml_input }
    }
}

#[derive(Clone, Debug)]
pub struct DropTypeInputLabel {
    dt_in_lb: TextDisplay,
}

impl DropTypeInputLabel {
    pub fn new(x: i32, y: i32, label: &str) -> Self {
        let mut dt_in_lb = TextDisplay::new(x, y, 0, 0, None).with_label(label);
        dt_in_lb.clear_visible_focus();
        dt_in_lb.set_label_color(Color::White);
        dt_in_lb.set_label_size(15);
        dt_in_lb.set_align(Align::TopLeft);

        Self { dt_in_lb }
    }
}

#[derive(Clone, Debug)]
pub struct DropTypeUnder {
    dt_und: Frame,
}

impl DropTypeUnder {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        let mut dt_und = Frame::new(x, y, w, h, None);
        dt_und.clear_visible_focus();
        dt_und.set_color(Color::White);
        dt_und.set_frame(FrameType::RoundedBox);
        Self { dt_und }
    }
}

#[derive(Clone, Debug)]
pub struct CRUDButton {
    crd_btn: Button,
}

impl CRUDButton {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> Self {
        let mut crd_btn = Button::new(x, y, w, h, None).with_label(&label.to_uppercase());
        crd_btn.clear_visible_focus();
        crd_btn.set_frame(FrameType::FlatBox);
        crd_btn.set_color(Color::DarkYellow);
        crd_btn.set_selection_color(Color::Cyan);
        crd_btn.set_label_color(Color::Black);
        crd_btn.handle(move |_b, ev| match ev {
            Event::Enter => {
                set_cursor(Cursor::Hand);
                redraw();
                true
            }
            Event::Leave => {
                set_cursor(Cursor::Default);
                redraw();
                true
            }
            Event::Push => {
                set_cursor(Cursor::Hand);
                redraw();
                true
            }
            _ => false,
        });
        Self { crd_btn }
    }
}

pub struct ReturnButton {
    r_btn: Button,
}

impl ReturnButton {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> Self {
        let mut r_btn = Button::new(x, y, w, h, None).with_label(&label.to_uppercase());
        r_btn.set_label_color(Color::White);
        r_btn.set_label_size(15);
        r_btn.set_color(Color::BackGround);
        r_btn.set_frame(FrameType::FlatBox);
        r_btn.clear_visible_focus();
        r_btn.handle(move |_b, ev| match ev {
            Event::Push => {
                set_cursor(Cursor::Hand);
                redraw();
                true
            }
            Event::Enter => {
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
        Self { r_btn }
    }
}
