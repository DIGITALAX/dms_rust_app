use crate::widgets::{
    droptypes::{DropTypeInput, DropTypeMultiInput},
    sidebar::MenuButton,
};
use fltk::button::Button;
use mongodb::Database;

#[derive(Clone, Debug)]
pub enum Message {
    Start,
    MenuSelect(
        MenuButton,
        MenuButton,
        MenuButton,
        MenuButton,
        MenuButton,
        Option<Database>,
    ),
    DropTypes(Option<Database>),
    Ready(Option<Database>),
    DropTypeModify(Button),
    ProductModify(Button),
    DropTypeAdd(DropTypeInput, DropTypeMultiInput),
    DropTypeDelete(DropTypeInput),
    DropTypeUpdate,
    ReturnDropType,
    DropTypeNew,
    DropProducts(Option<Database>, String),
    Error,
}
