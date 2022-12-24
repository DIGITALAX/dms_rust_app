use fltk::frame::Frame;
use mongodb::Database;
use crate::widgets::sidebar::MenuButton;

#[derive(Clone)]
pub enum Message {
    Start,
    MenuSelect(MenuButton, MenuButton, MenuButton, MenuButton, MenuButton, Option<Database>),
    DropTypes(Option<Database>),
    Ready(Option<Database>),
    DropTypeModify(Frame),
    Error,
}
