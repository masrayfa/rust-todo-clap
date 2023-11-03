use dotenvy_macro::dotenv;
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, Set, ActiveValue, ActiveModelTrait};
use uuid::Uuid;

use crate::db;
use crate::args::{TodoCommand, TodoSubCommand, CreateTodo, UpdateTodo, DeleteEntity, DeleteTodo};

pub fn handle_todo_command(todo: TodoCommand) {
    let command = todo.command;

    match command {
        TodoSubCommand::Create(todo) => {
            create_todo(todo);
        },
        TodoSubCommand::Update(todo) => {
            update_todo(todo);
        }
        TodoSubCommand::Delete(todo) => {
            delete_todo(todo);
        }
        TodoSubCommand::Read => {
            read_todos();
        }
    }
}

fn create_todo(todo: CreateTodo) {
    todo!()
}

fn update_todo(todo: UpdateTodo) {
    todo!()
}

fn delete_todo(todo: DeleteTodo) {
    todo!()
}

fn read_todos() {
    todo!()
}

