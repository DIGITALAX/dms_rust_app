use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Currency {
    USD,
    USDT,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub name: String,
    pub description: String,
    pub collection: String,
    pub quantity: u32,
    pub images: String,
    pub slug: String,
    pub price: u32,
    pub currency: Currency,
    pub amount_sold: u32,
    pub sold_out: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Collection {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Admin {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    username: String,
    password: String,
}
