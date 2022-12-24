use fltk::frame::Frame;
use mongodb::Database;
use crate::widgets::{sidebar::MenuButton, droptypes::DropTypeFrame};

#[derive(Clone)]
pub enum Message {
    Start,
    MenuSelect(MenuButton, MenuButton, MenuButton, MenuButton, MenuButton, Option<Database>),
    DropTypes(Option<Database>),
    Ready(Option<Database>),
    DropTypeModify(DropTypeFrame),
    Error,
}
