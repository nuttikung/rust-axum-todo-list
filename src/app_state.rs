use sqlx::PgPool;

#[derive(Debug)]
pub struct AppState {
    pub connection: PgPool,
}
