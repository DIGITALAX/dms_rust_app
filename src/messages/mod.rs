use mongodb::Database;
use crate::{widgets::MenuButton};

// add generic T meaning that we could h
#[derive(Clone)]
pub enum Message {
    Start,
    MenuSelect(MenuButton, MenuButton, MenuButton, MenuButton, MenuButton, Option<Database>),
    DropTypes(Option<Database>),
    Ready(Option<Database>),
    Error,
}
