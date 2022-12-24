use crate::schemas::DropType;
use fltk::{
    app::redraw,
    button::Button,
    draw::set_cursor,
    enums::{Align, Color, Cursor, Event, Font, FrameType},
    frame::Frame,
    group::Scroll,
    input::{Input, MultilineInput},
    misc::Progress,
    prelude::*,
    text::TextDisplay,
    widget_extends,
};
use num_integer::{div_ceil, div_floor, div_rem};
widget_extends!(DropTypeFrame, Frame, dt_frame);

pub struct DropTypeFrame {
    dt_frame: Frame,
}

impl DropTypeFrame {
    pub fn new(x: i32, y: i32, w: i32, h: i32, title: &str) -> Self {
        let mut label = String::new();
        if title.len() > 30 {
            label = title[0..=30].to_string() + "...";
        } else {
            label = title.to_string();
        }
        let mut dt_frame = Frame::new(x, y, w, h, None).with_label(&label);
        dt_frame.clear_visible_focus();
        dt_frame.set_frame(FrameType::RoundedBox);
        dt_frame.set_color(Color::Cyan);
        dt_frame.set_selection_color(Color::Cyan);
        dt_frame.set_align(Align::Center);
        dt_frame.set_label_color(Color::Black);
        dt_frame.handle(move |b, ev| match ev {
            Event::Enter => {
                set_cursor(Cursor::Hand);
                true
            }
            Event::Focus => {
                set_cursor(Cursor::Hand);
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
    drop_frame_size: i32,
    row_height: i32,
    col_width: i32,
    mut x_pos: i32,
    mut y_pos: i32,
    droptypes: &Vec<DropType>,
) {
    let mut title_incramentor = 0;
    for row in 0..div_ceil(droptypes_len, number_of_cols) as i32 {
        for col in 0..number_of_cols {
            if title_incramentor < droptypes_len {
                let result = DropTypeFrame::new(
                    x_pos,
                    y_pos,
                    drop_frame_size,
                    drop_frame_size,
                    &droptypes[title_incramentor as usize].title,
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
