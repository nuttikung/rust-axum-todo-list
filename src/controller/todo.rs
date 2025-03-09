use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::Utc;
use serde_json::{Value, json};

use crate::{
    app_state::AppState,
    models::todo::{CreateTodo, Todo, UpdateTodo},
};

// region :      --- Todo Handler
pub async fn list_todo(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let result = sqlx::query_as::<_, Todo>(r#"select * from todos order by id"#)
        .fetch_all(&state.connection)
        .await;
    match result {
        Ok(todo) => return Ok(Json(json!({ "data": todo }))),
        Err(e) => {
            tracing::error!("Database error: {:?}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "internal server error"
                })),
            ));
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
        Ok(id) => return Ok((StatusCode::CREATED, Json(json!({ "id": id })))),
        Err(e) => {
            tracing::error!("Database error: {:?}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "internal server error"
                })),
            ));
        }
    }
}

pub async fn update_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateTodo>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    // TODO: refactor this update_todo and delete_todo both are using this statement.
    let record = sqlx::query_as::<_, Todo>(r#"select * from todos where id=($1)"#)
        .bind(id)
        .fetch_optional(&state.connection)
        .await
        .map_err(|e| {
            tracing::error!("Database error: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "internal server error"
                })),
            );
        })?;
    // TODO: refactor this update_todo and delete_todo both are using this statement.
    if record.is_none() {
        tracing::error!("Todo ID:{} does not exists", id);
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": "record does not exists"
            })),
        ));
    }
    // UPDATE ROW VIA SQL
    let updated_at = Utc::now();
    let result = sqlx::query_scalar::<_, i64>(r#"update todos set description=($1), completed=($2), updated_at=($3) where id=($4) returning id"#)
    .bind(payload.description)
    .bind(payload.completed)
    .bind(updated_at)
    .bind(id)
    .fetch_one(&state.connection)
    .await;
    // Return Result
    match result {
        Ok(id) => return Ok((StatusCode::OK, Json(json!({ "id": id })))),
        Err(e) => {
            tracing::error!("Database error: {:?}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "internal server error"
                })),
            ));
        }
    }
}

pub async fn delete_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // TODO: refactor this
    let record = sqlx::query_as::<_, Todo>(r#"select * from todos where id=($1)"#)
        .bind(id)
        .fetch_optional(&state.connection)
        .await
        .map_err(|e| {
            tracing::error!("Database error: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "internal server error"
                })),
            );
        })?;
    // TODO: refactor this
    if record.is_none() {
        tracing::error!("Todo ID:{} does not exists", id);
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": "record does not exists"
            })),
        ));
    }

    let result = sqlx::query(r#"delete from todos where id=($1)"#)
        .bind(id)
        .execute(&state.connection)
        .await;

    match result {
        Ok(_) => return Ok(Json(json!({ "message": "success" }))),
        Err(e) => {
            tracing::error!("Database error: {:?}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "internal server error"
                })),
            ));
        }
    }
}
// end region :  --- Todo Handler
