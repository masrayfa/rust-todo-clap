use chrono::Utc;
use dotenvy_macro::dotenv;
use sea_orm::{EntityTrait, Set, ActiveModelTrait, Statement, DbBackend};
use uuid::Uuid;

use crate::db;
use crate::args::{TodoCommand, TodoSubCommand, CreateTodo, UpdateTodo, DeleteTodo};
use crate::helper::parse_date::parse_date;

pub async fn handle_todo_command(todo: TodoCommand) {
    let command = todo.command;

    match command {
        TodoSubCommand::Create(todo) => {
            create_todo(todo).await;
        },
        TodoSubCommand::Update(todo) => {
            update_todo(todo).await;
        }
        TodoSubCommand::Delete(todo) => {
            delete_todo(todo).await;
        }
        TodoSubCommand::Read => {
            read_todos().await;
        }
        TodoSubCommand::ReadByUserId(todo) => {
            read_todos_by_user_id(todo.user_id).await;
        }
    }
}

// Create a new todo
async fn create_todo(todo: CreateTodo) {
    println!("Creating todo: {:?}", todo);

    let database_uri = dotenv!("DATABASE_URL");
    let db = db::establish_connection(database_uri).await;

    let id = Uuid::new_v4();

    let now = Utc::now();

    let duedate = parse_date(&todo.duedate);

    match db {
        Ok(db) => {
            let new_todo = entity::todo::ActiveModel {
                id: Set(id.to_string()),
                title: Set(todo.title),
                task: Set(todo.task),
                created_at: Set(now.naive_utc()),
                due_date: Set(duedate),
                user_id: Set(todo.user_id),
                ..Default::default()
            };

            let todo = new_todo.insert(&db).await.unwrap();
            println!("Todo created: {:?}", todo);
        },
        Err(err) => {
            eprint!("Failed to establish a database connection and create todo: {}", err)
        }
    }
}

// Update an existing todo
async fn update_todo(todo: UpdateTodo) {
    println!("Updating todo: {:?}", todo);

    let database_uri = dotenv!("DATABASE_URL");
    let db = db::establish_connection(database_uri).await;

    let id = Uuid::parse_str(&todo.id).unwrap();

    type TodoModel = entity::todo::Model;

    let now = Utc::now();

    let duedate = parse_date(&todo.duedate);

    match db {
        Ok(db) => {
            let old_todo : Option<TodoModel> = entity::todo::Entity::find_by_id(id.clone())
                .one(&db)
                .await
                .unwrap(); 

            // let mut user_active_model : entity::user::ActiveModel = find_user.unwrap().into();
            let todo_active_model : entity::todo::ActiveModel = old_todo.unwrap().into();

            let updated_todo = entity::todo::ActiveModel {
                id: Set(id.to_string()),
                title: Set(todo.title),
                task: Set(todo.task),
                created_at: Set(now.naive_utc()),
                due_date: Set(duedate),
                user_id: Set(todo.user_id),
                ..todo_active_model
            };

            println!("Todo updated: {:?}", updated_todo);
        }
        Err(err) => {
            eprint!("Failed to establish a database connection and update todo: {}", err)
        }
    }
}

async fn delete_todo(todo: DeleteTodo) {
    println!("Deleting todo: {:?}", todo);

    let database_uri = dotenv!("DATABASE_URL");
    let db = db::establish_connection(database_uri).await;

    let id = Uuid::parse_str(&todo.id).unwrap();

    match db {
        Ok(db) => {
            let deleted_todo = entity::todo::Entity::find_by_id(id.clone())
                .one(&db)
                .await
                .unwrap(); 

            let deleted_todo: entity::todo::Model = deleted_todo.unwrap().into();

            println!("Todo deleted: {:?}", deleted_todo);
        },
        Err(err) => {
            eprint!("Failed to establish a database connection and delete todo: {}", err)
        }
    }
}

async fn read_todos() {
    println!("Reading todos");

    let database_uri = dotenv!("DATABASE_URL");
    let db = db::establish_connection(database_uri).await;

    match db {
        Ok(db) => {
            let todos = entity::todo::Entity::find()
                .all(&db)
                .await
                .unwrap();

            println!("Todos: {:?}", todos);
        },
        Err(err) => {
            eprint!("Failed to establish a database connection and read todos: {}", err)
        }
    }
}

async fn read_todos_by_user_id(user_id: String) {
    println!("Reading todos by user id: {}", user_id);

    let database_uri = dotenv!("DATABASE_URL");
    let db = db::establish_connection(database_uri).await;

    match db {
        Ok(db) => {
            let todos = entity::todo::Entity::find()
            .from_raw_sql(Statement::from_sql_and_values(
                DbBackend::Postgres,
                 r#"SELECT * FROM public.todo WHERE user_id = $1"#, 
                 [user_id.into()]))
                 .all(&db)
                 .await;

            println!("Todos: {:?}", todos);
        }
        Err(err) => {
            eprint!("Failed to establish a database connection and read todos by user id: {}", err)
        }
    }
}