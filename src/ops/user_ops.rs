use dotenvy_macro::dotenv;
use sea_orm::{EntityTrait, Set, ActiveModelTrait, DeleteResult, DbConn, DbErr};
use uuid::Uuid;

use crate::db::{self, establish_connection};
use crate::args::{CreateUser, UpdateUser, DeleteEntity, UserSubCommand, UserCommand};

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

    let id = Uuid::new_v4();

    let new_user = entity::user::ActiveModel {
        id: Set(id.to_string()),
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
            eprint!("Failed to establish a database connection and create user: {}", err)
        }
    }

}

async fn update_user(user: UpdateUser) {
    println!("Updating user: {:?}", user);

    let database_uri = dotenv!("DATABASE_URL");
    let db = db::establish_connection(database_uri).await;

    let id = Uuid::parse_str(&user.id).unwrap();

    type UserModel = entity::user::Model;

    match db {
        Ok(db) => {
            let find_user: Option<UserModel> = entity::user::Entity::find_by_id(id.clone())
                .one(&db)
                .await
                .unwrap();
            let user_active_model : entity::user::ActiveModel = find_user.unwrap().into();

            let updated_user = entity::user::ActiveModel {
                id: Set(id.to_string()),
                name: Set(user.name),
                email: Set(user.email),
                ..user_active_model
            };

            let updated_user: entity::user::Model = updated_user.update(&db).await.unwrap();

            println!("User updated: {:?}", updated_user);
        },

        Err(err) => {
            eprint!("Failed to establish a database connection and update user: {}", err)
        }
    }

}

async fn delete_user(user: DeleteEntity) {
    println!("Deleting user: {:?}", user);

    let database_uri = dotenv!("DATABASE_URL");
    let db = establish_connection(database_uri).await;

    let id = Uuid::parse_str(&user.id).unwrap();

    match db {
        Ok(db) => {
            let res: DeleteResult = entity::user::Entity::delete_by_id(id.clone())
                .exec(&db)
                .await
                .unwrap();

            println!("User deleted: {:?}", res);
        }
        Err(err) => {
            eprint!("Failed to establish a database connection and delete user: {}", err)
        }  
    }
}

async fn show_users() {
   println!("Showing users"); 

   let database_uri = dotenv!("DATABASE_URL");
   let db: Result<DbConn, DbErr> = establish_connection(database_uri).await;

    match db {
         Ok(db) => {
              let users: Vec<entity::user::Model> = entity::user::Entity::find()
                .all(&db)
                .await
                .unwrap();
    
              println!("Users: {:?}", users);
         },
         Err(err) => {
              eprint!("Failed to establish a database connection and show users: {}", err)
         }
    }
}