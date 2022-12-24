use dotenv::dotenv;
use mongodb::{bson::doc, options::ClientOptions, Client};
use std::env;

pub(crate) async fn connect() -> mongodb::error::Result<mongodb::Database> {
    dotenv().ok();
    let connection_url = env::var("MONGO_URL").ok().unwrap();
    let client_options = ClientOptions::parse(connection_url).await?;
    let db = Client::with_options(client_options)?.database("test");
    db.run_command(doc! {"ping": 1u32}, None).await?;

    Ok(db)
}
