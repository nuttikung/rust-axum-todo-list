use sqlx::{Pool, Postgres};

#[derive(Debug)]
pub struct AppState {
    pub connection: Pool<Postgres>,
}
