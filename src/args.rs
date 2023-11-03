use chrono::{DateTime, Utc};
use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct PagaweanArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Create, Update, Delete, Read user
    User(UserCommand),

    // Create, Update, Delete, Read todos
    Todo(TodoCommand),
}

#[derive(Debug, Args)]
pub struct UserCommand {
    #[clap(subcommand)]
    pub command: UserSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum UserSubCommand {
    Create(CreateUser),

    Update(UpdateUser),

    Delete(DeleteEntity),

    Show,
}


#[derive(Debug, Args)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}
#[derive(Debug, Args)]
pub struct UpdateUser {
    pub id: String,
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Args)]
pub struct DeleteEntity {
    pub id: String,
}

#[derive(Debug, Args)]
pub struct TodoCommand {
    #[clap(subcommand)]
    pub command: TodoSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum TodoSubCommand {
    Create(CreateTodo),

    Update(UpdateTodo),

    Read,

    Delete(DeleteTodo),
}

#[derive(Debug, Args)]
pub struct CreateTodo {
    title: String,
    task: String,
    duedate: DateTime<Utc>,
}

#[derive(Debug, Args)]
pub struct UpdateTodo {
    pub id: String,
    pub title: Option<String>,
    pub task: Option<String>,
    pub duedate: Option<DateTime<Utc>>,
}

#[derive(Debug, Args)]
pub struct DeleteTodo {
    pub id: String,
}
