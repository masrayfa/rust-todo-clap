mod args;
mod db;
mod ops;
mod models;

use dotenvy_macro::dotenv;

use args::EntityType;
use args::PagaweanArgs;
use clap::Parser;
use ops::{todo_ops, user_ops};

#[tokio::main]
async fn main() {
    let args = PagaweanArgs::parse();

    match args.entity_type {
        EntityType::User(user) => user_ops::handle_user_command(user).await,
        EntityType::Todo(todo) => todo_ops::handle_todo_command(todo)
    }
}
