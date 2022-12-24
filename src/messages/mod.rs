use crate::schemas::Collection;
use mongodb::Database;

// add generic T meaning that we could h
#[derive(Clone)]
pub enum Message {
    Start,
    Ready(Option<Database>),
    // Reset(Collection),
    // Insert(String),
    // Update(i32),
    // Delete(String),
    Error,
}
