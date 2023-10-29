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
    /// Create, update delete, read user
    User(UserCommand),

    // Create, update, delete, read todos
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
    pub id: i32,
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Args)]
pub struct DeleteEntity {
    pub id: i32,
}

#[derive(Debug, Args)]
pub struct TodoCommand {
    #[clap(subcommand)]
    command: TodoSubCommand,
}

#[derive(Debug, Subcommand)]
enum TodoSubCommand {
    Create(CreateTodo),

    Update(UpdateTodo),

    Read,

    Delete(DeleteTodo),
}

#[derive(Debug, Args)]
struct CreateTodo {
    title: String,
    action: String,
    duedate: DateTime<Utc>,
}

#[derive(Debug, Args)]
struct UpdateTodo {
    id: i32,
    title: Option<String>,
    action: Option<String>,
    duedate: Option<DateTime<Utc>>,
}

#[derive(Debug, Args)]
struct DeleteTodo {
    id: i32,
}
