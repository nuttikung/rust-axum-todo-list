use std::sync::Arc;

use axum::Router;
use axum::http::Method;
use axum::routing::{get, put};
use rust_axum_todo_list::app_state::AppState;
use rust_axum_todo_list::controller::todo::{add_todo, delete_todo, list_todo, update_todo};
use rust_axum_todo_list::database;
use rust_axum_todo_list::setting::Setting;

use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::decompression::RequestDecompressionLayer;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    // region :      --- Tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    // end region :  --- Tracing

    // region :      --- Set variables from environment variables
    let setting = Setting::new().unwrap();
    // end region :  --- Set variables from environment variables

    // region :      --- Create database pool
    let db_pool = AppState {
        connection: database::conn_getting(Arc::clone(&setting))
            .await
            .expect("can't connect to database"),
    };
    tracing::debug!("database connection has been established.");
    // end region :  --- Create database pool

    // region :      --- All Route
    let todo_router = todo_routes();
    // end region :  --- All Route

    // region :      --- Main Router
    let router = Router::new()
        .layer(
            CorsLayer::new()
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                ])
                .allow_origin(Any),
        )
        .layer(TraceLayer::new_for_http())
        .layer(
            ServiceBuilder::new()
                .layer(RequestDecompressionLayer::new())
                .layer(CompressionLayer::new()),
        )
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/api/todos", todo_router)
        .with_state(Arc::new(db_pool));
    // end region :  --- Main Router

    // region :      --- Create TCP listener
    let port: String = setting.server.port.to_string();
    let host: String = String::from("127.0.0.1");
    let address = format!("{}:{}", host, &port);
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    tracing::debug!("--> LISTENING on {:?} \n", listener.local_addr());
    // end region :  --- Create TCP listener

    // region :      --- Serve the application
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
    // end region :  --- Serve the application
}

// TODO: move to route file
// region :      --- Todo Routes
fn todo_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_todo).post(add_todo))
        .route("/{id}", put(update_todo).delete(delete_todo))
}
// end region :  --- Todo Routes

// region :      ---
// Git (Get All) ->
// ALL -> []
// 6
// (P, Dorin, Fuse)
// DETAIL (id) -> {
//     completed": true,
//     created_at: "2025-03-05T17:38:48.103054Z",
//     description: "Nat first todo",
//     id: 1,
//     updated_at: "2025-03-05T17:38:48.103054Z"
// }
//
// CREATE -> {
//     description: string
// }

// (Ton Great)
// UPDATE (id) -> {
//     description: string
//     completed: boolean
// }

// (Jeff Theng)
// DELETE (id)
// end region :  ---
