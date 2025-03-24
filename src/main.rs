use std::sync::Arc;

use axum::Router;
use axum::http::HeaderValue;
use axum::http::header::CONTENT_TYPE;
use axum::routing::get;
use rust_axum_todo_list::app_state::AppState;
use rust_axum_todo_list::controller::todo::{
    add_todo, delete_todo, detail_todo, list_todo, update_todo,
};
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

    // region :      --- Middleware Layer
    let origins = [
        "http://localhost:8080".parse::<HeaderValue>().unwrap(),
        "https://codesandbox.io".parse::<HeaderValue>().unwrap(),
        "https://7vyyx9.csb.app".parse::<HeaderValue>().unwrap(),
        "https://n6d8k5.csb.app".parse::<HeaderValue>().unwrap(),
        "https://9lp4sn.csb.app".parse::<HeaderValue>().unwrap(),
    ];

    let cors_layer = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers([CONTENT_TYPE])
        .allow_origin(origins);

    let trace_layer = TraceLayer::new_for_http();

    let services_layer = ServiceBuilder::new()
        .layer(RequestDecompressionLayer::new())
        .layer(CompressionLayer::new());
    // end region :  --- Middleware Layer

    // region :      --- All Route
    let todo_router = todo_routes();
    // end region :  --- All Route

    // region :      --- Main Router
    let router = Router::new()
        // Health Check for monitoring.
        .route("/", get(|| async { "OK!" }))
        .nest("/api/todos", todo_router)
        .layer(cors_layer)
        .layer(trace_layer)
        .layer(services_layer)
        .with_state(Arc::new(db_pool));
    // end region :  --- Main Router

    // region :      --- Create TCP listener
    let port: String = setting.server.port.to_string();
    let host: String = String::from("0.0.0.0");
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
        .route(
            "/{id}",
            get(detail_todo).put(update_todo).delete(delete_todo),
        )
}
// end region :  --- Todo Routes
