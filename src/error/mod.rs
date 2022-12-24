use mongodb::{bson,bson::raw::ValueAccessError};
use std::env::VarError::{NotPresent, NotUnicode};

#[derive(Debug,Clone)]
pub enum MyError {
    MongoErr(mongodb::error::Error),
    BsonErr(bson::ser::Error),
    ValueAccErr(ValueAccessError),
    VarErr(String),
    OtherErr(String)
}

impl MyError {
   pub fn new(err: &str) -> Self {
        Self::OtherErr(err.to_owned())
    }
}

impl Default for MyError {
    fn default() -> Self {
        Self::OtherErr("Not implemented error".to_owned())
    }
}

impl ToString for MyError {
    fn to_string(&self) -> String {
        match self {
            Self::MongoErr(err) => {err.to_string()}
            Self::BsonErr(err) => {err.to_string()}
            Self::ValueAccErr(err) => {err.to_string()}
            Self::VarErr(err) => {err.to_owned()}
            Self::OtherErr(err) => {err.to_owned()}
        }
    }
}

impl From<mongodb::error::Error> for MyError {
    fn from(err: mongodb::error::Error) -> Self {
        Self::MongoErr(err)
    }
}

impl From<bson::ser::Error> for MyError {
    fn from(err: bson::ser::Error) -> Self {
        Self::BsonErr(err)
    }
}

impl From<std::env::VarError> for MyError {
    fn from(err: std::env::VarError) -> Self {
        match err {
            NotPresent => Self::VarErr("NotPresent".to_string()),
            NotUnicode(s) => Self::VarErr("NotUnicode: ".to_string() + s.to_str().unwrap()),
        }
    }
}

impl From<ValueAccessError> for MyError {
    fn from(err: ValueAccessError) -> Self {
        Self::ValueAccErr(err)
    }
}

impl From<tokio::task::JoinError> for MyError {
    fn from(_: tokio::task::JoinError) -> Self {
        Self::OtherErr("Join Error".to_string())
    }
}
