use crate::schemas::{DropType, Product, User};
use fltk::{
    app::{redraw, set_background2_color, set_background_color, set_color, set_font, App},
    button::{Button, CheckButton},
    draw::set_cursor,
    enums::{Align, Color, Cursor, Event, Font, FrameType},
    frame::Frame,
    group::{Scroll, ScrollType},
    input::{Input, MultilineInput},
    prelude::*,
    table,
    table::{Table, TableContext},
    text::{TextBuffer, TextDisplay},
    window::Window,
};
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, Bson},
    options::FindOptions,
    results::InsertOneResult,
    Database,
};

pub async fn get_product(
    database: Option<Database>,
    _name: String,
) -> mongodb::error::Result<Vec<Product>> {
    let mut result: Vec<Product> = Vec::new();
    match database {
        Some(db) => {
            let products = db.collection::<Product>("products");
            // doc transforms what we insert into a filter to convert into bson, for insert we just do product and serialize/de does the job for us
            // find  gives back an iterator over the results which is cursor
            let mut cursor = products
                .find(doc! {"name": _name.to_lowercase()}, None)
                .await?;
            while let Some(product) = cursor.try_next().await? {
                result.push(product);
            }
        }
        None => {}
    }
    Ok(result)
}

pub async fn get_collection(
    database: Option<Database>,
    _title: String,
) -> mongodb::error::Result<Vec<DropType>> {
    let mut result: Vec<DropType> = Vec::new();
    println!("here {}", _title);
    match database {
        Some(db) => {
            let collections = db.collection::<DropType>("droptypes");
            let mut cursor = collections
                .find(doc! {"title": _title.to_lowercase()}, None)
                .await?;
            while let Some(collection) = cursor.try_next().await? {
                result.push(collection);
            }
        }
        None => {}
    }
    Ok(result)
}

pub async fn get_user(
    database: Option<Database>,
    _username: String,
) -> mongodb::error::Result<Vec<User>> {
    let mut result: Vec<User> = Vec::new();
    match database {
        Some(db) => {
            let users = db.collection::<User>("users");
            let mut cursor = users.find(doc! {"username": _username}, None).await?;

            while let Some(user) = cursor.try_next().await? {
                result.push(user);
            }
        }
        None => {}
    }
    Ok(result)
}

pub async fn get_products(
    database: Option<Database>,
) -> mongodb::error::Result<Vec<Product>> {
    let mut result: Vec<Product> = Vec::new();
    match database {
        Some(db) => {
            let products = db.collection::<Product>("products");
            // let find_opt = FindOptions::builder().sort(doc! { "name": 1i32 }).build();
            let mut cursor = products.find(doc! {}, None).await?;
            while let Some(product) = cursor.try_next().await? {
                result.push(product);
            }
        }
        None => {}
    }
    Ok(result)
}

pub async fn get_collections(
    database: Option<Database>,
) -> mongodb::error::Result<Vec<DropType>> {
    let mut result: Vec<DropType> = Vec::new();
    match database {
        Some(db) => {
            let collections = db.collection::<DropType>("droptypes");
            let find_opt = FindOptions::builder().sort(doc! { "name": 1i32 }).build();
            let mut cursor = collections.find(doc! {}, find_opt).await?;
            while let Some(collection) = cursor.try_next().await? {
                result.push(collection);
            }
        }
        None => {}
    }
    Ok(result)
}

pub async fn get_users(database: Option<Database>) -> mongodb::error::Result<Vec<User>> {
    let mut result: Vec<User> = Vec::new();
    match database {
        Some(db) => {
            let users = db.collection::<User>("users");
            let find_opt = FindOptions::builder()
                .sort(doc! { "username": 1i32 })
                .build();
            let mut cursor = users.find(doc! {}, find_opt).await?;

            while let Some(user) = cursor.try_next().await? {
                result.push(user);
            }
        }
        None => {}
    }
    Ok(result)
}

pub async fn add_product(
    database: Option<Database>,
    _product: Product,
) -> mongodb::error::Result<String> {
    let mut result = String::new();
    match database {
        Some(db) => {
            let products = db.collection::<Product>("products");
            // ? after await verifies already that product was added or not
            let product: InsertOneResult = products.insert_one(_product, None).await?;
            // let a = product.inserted_id;
            // println!(a)
        }
        None => {}
    }
    Ok(result)
}

pub async fn add_collection(
    database: Option<Database>,
    _collection: DropType,
) -> mongodb::error::Result<Vec<DropType>> {
    let mut result = Vec::new();
    let mut id = String::new();
    match database {
        Some(db) => {
            let collections = db.collection::<DropType>("droptypes");
            let collection = collections.insert_one(_collection, None).await?;
            let a = collection.inserted_id;
            match a {
                Bson::ObjectId(s) => {
                    id = s.to_hex();
                }
                _ => {}
            }
            let mut cursor = collections.find(doc! {"_id": a }, None).await?;
            while let Some(collection) = cursor.try_next().await? {
                result.push(collection);
            }
        }
        None => {}
    }
    Ok(result)
}

pub async fn delete_collection(
    database: Option<Database>,
    _title: String,
) -> mongodb::error::Result<String> {
    let mut result = String::new();
    match database {
        Some(db) => {
            let collections = db.collection::<DropType>("droptypes");
            println!("title {:?}", _title);
            let collection = collections
                .find_one_and_delete(doc! {"title": _title.to_lowercase()}, None)
                .await?;
            println!("collection {:?}", collection);
            result = "deleted".to_string();
        }
        None => {}
    }
    Ok(result)
}
