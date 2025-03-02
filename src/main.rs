
use axum::{response::IntoResponse, routing::get, Router};
use rust_axum_todo_list::setting::Setting;

#[tokio::main]
async fn main(){
    // region :      --- Router Constant
    let api_routes = Router::new().merge(route_todo());
    // Make it nested as /api/*
    let router = Router::new().nest("/api", api_routes);
    // end region :  --- Router Constant

    // region :      --- Load Setting
    let setting = Setting::new().unwrap();
    let port: String = setting.server.port.to_string();
    let host: String = String::from("127.0.0.1");
    let address = format!("{}:{}", host, &port ) ;
    // end region :  --- Load Setting

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    println!("--> LISTENING on {:?} \n", listener.local_addr());
    // region : --------start server--------
    axum::serve(listener, router.into_make_service()).await.unwrap();
}

// region :      --- Route Hello
fn route_todo() -> Router {
    return Router::new()
        .route("/hello", get(handler_hello))
}
// end region :  --- Route Hello

// region :      --- Handle Hello
async fn handler_hello() -> impl IntoResponse {
    return "Hello World!";
}
// end region :  --- Handle Hello
