use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use serde_json::{Value, json};

use crate::{
    app_state::AppState,
    models::todo::{CreateTodo, Todo},
};

// region :      --- Todo Handler
pub async fn list_todo(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let result = sqlx::query_as::<_, Todo>(r#"select * from todos order by id"#)
        .fetch_all(&state.connection)
        .await;
    match result {
        Ok(todo) => Ok(Json(json!({ "data": todo }))),
        Err(e) => {
            tracing::error!("Database error: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "internal server error"
                })),
            ))
        }
    }
}

pub async fn add_todo(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    let result =
        sqlx::query_scalar::<_, i64>(r#"insert into todos (description) values ($1) returning id"#)
            .bind(payload.description)
            .fetch_one(&state.connection)
            .await;

    match result {
        Ok(id) => Ok((StatusCode::CREATED, Json(json!({ "id": id })))),
        Err(e) => {
            tracing::error!("Database error: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "internal server error"
                })),
            ))
        }
    }
}
// end region :  --- Todo Handler
