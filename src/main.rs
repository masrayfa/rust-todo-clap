mod args;
mod db;

use sea_orm::{Database, DatabaseConnection};
use dotenvy_macro::dotenv;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let database_uri = dotenv!("DATABASE_URL");
    db::establish_connection(database_uri).await;
}
