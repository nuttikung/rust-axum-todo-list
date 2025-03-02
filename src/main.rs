
use axum::{response::IntoResponse, routing::get, Router};
use rust_axum_todo_list::setting::Setting;

#[tokio::main]
async fn main(){
    let routes_hello = Router::new().route("/hello", get(handler_hello));

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
    axum::serve(listener, routes_hello.into_make_service()).await.unwrap();
}

// region :      --- Handle Hello
async fn handler_hello() -> impl IntoResponse {
    return "Hello World!";
}
// end region :  --- Handle Hello
