use mongodb::Database;
use crate::widgets::MenuButton;

// add generic T meaning that we could h
#[derive(Clone)]
pub enum MainMessage {
    Start,
    MenuSelect(MenuButton, MenuButton, MenuButton, MenuButton, MenuButton),
    Ready(Option<Database>),
    Error,
}

pub enum DropTypeMessage {
    Start,
    Ready(Option<Database>),
    // Reset(Collection),
    // Insert(String),
    // Update(i32),
    // Delete(String),
    Error,
}
