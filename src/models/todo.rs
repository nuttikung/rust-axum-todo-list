use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct Todo {
    pub id: i64,
    pub description: String,
    pub completed: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize, Debug)]
pub struct CreateTodo {
    pub description: String,
}

pub type TodoList = Vec<Todo>;
