use mongodb::Database;
use fltk::button::Button;
use crate::widgets::sidebar::MenuButton;

#[derive(Clone, Debug)]
pub enum Message {
    Start,
    MenuSelect(MenuButton, MenuButton, MenuButton, MenuButton, MenuButton, Option<Database>),
    DropTypes(Option<Database>),
    Ready(Option<Database>),
    DropTypeModify(Button),
    DropTypeAdd,
    DropTypeUpdate,
    ReturnDropType,
    Error,
}
