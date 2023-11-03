use uuid::{Uuid};
use chrono::{DateTime, Utc};

pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub task: String,
    pub due_date: DateTime<Utc>,
}