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

    /// Create, Update, Delete, Read todos
    Todo(TodoCommand),
}

#[derive(Debug, Args)]
pub struct UserCommand {
    #[clap(subcommand)]
    pub command: UserSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum UserSubCommand {
    /// Create a new user
    Create(CreateUser),

    /// Update an existing user
    Update(UpdateUser),

    /// Delete an existing user
    Delete(DeleteEntity),

    /// Show all users
    Show,
}


#[derive(Debug, Args)]
pub struct CreateUser {
    /// name of the user
    pub name: String,

    /// email of the user
    pub email: String,
}
#[derive(Debug, Args)]
pub struct UpdateUser {
    /// id of the user
    pub id: String,

    /// name of the user
    pub name: String,

    /// email of the user
    pub email: String,
}

#[derive(Debug, Args)]
pub struct DeleteEntity {
    /// id of the user
    pub id: String,
}

#[derive(Debug, Args)]
pub struct TodoCommand {
    #[clap(subcommand)]
    pub command: TodoSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum TodoSubCommand {
    /// Create a new todo
    Create(CreateTodo),

    /// Update an existing todo
    Update(UpdateTodo),

    /// Delete an existing todo
    Delete(DeleteTodo),

    /// Show all todos
    Read,
}

#[derive(Debug, Args)]
pub struct CreateTodo {
    /// title of the todo
    pub title: String,

    /// task of the todo
    pub task: String,

    /// due date of the todo
    pub duedate: String,

    /// user id of the todo
    pub user_id: String,
}

#[derive(Debug, Args)]
pub struct UpdateTodo {
    /// id of the todo
    pub id: String,

    /// title of the todo
    pub title: String,

    /// task of the todo
    pub task: String,

    /// due date of the todo
    pub duedate: String,

    /// user id of the todo
    pub user_id: String
}

#[derive(Debug, Args)]
pub struct DeleteTodo {
    /// id of the todo
    pub id: String,
}
