use dotenvy_macro::dotenv;
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, Set, ActiveValue, ActiveModelTrait};
use uuid::Uuid;

use crate::db;
use crate::args::{CreateUser, UpdateUser, DeleteEntity, UserSubCommand, UserCommand};
use crate::models::user_models::User;

pub async fn handle_user_command(user: UserCommand) {
    let command = user.command;

    match command {
        UserSubCommand::Create(user) => {
            create_user(user).await;
        },
        UserSubCommand::Update(user) => {
            update_user(user).await;
        }
        UserSubCommand::Delete(user) => {
            delete_user(user).await;
        }
        UserSubCommand::Show => {
            show_users().await;
        }
    }
}

async fn create_user(user: CreateUser) {
    println!("Creating user: {:?}", user);

    
    let database_uri = dotenv!("DATABASE_URL");
    let db = db::establish_connection(database_uri).await;

    let ID = Uuid::new_v4();

    let new_user = entity::user::ActiveModel {
        id: Set(ID.to_string()),
        name: Set(user.name),
        email: Set(user.email),
        ..Default::default()
    };

    match db {
        Ok(db) => {
            let user = new_user.insert(&db).await.unwrap();
            println!("User created: {:?}", user);
        },
        Err(err) => {
            eprint!("Failed to establish a database connection")
        }
    }

}

async fn update_user(user: UpdateUser) {
    todo!()
}

async fn delete_user(user: DeleteEntity) {
    todo!()
}

async fn show_users() {
    todo!()
}