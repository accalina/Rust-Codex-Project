use axum::{
    extract::Path,
    routing::get,
    Router,
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
struct UserResponse {
    message: String,
    success: bool,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/hello/{name}", get(hello_name));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8008").await.unwrap();
    println!("Listening on http://127.0.0.1:8008");
    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> Json<UserResponse> {
    Json (UserResponse {
        message: "Hello, world!".to_string(),
        success: true,
    })
}

async fn hello_name(Path(name): Path<String>) -> Json<UserResponse> {
    Json (UserResponse {
        message: format!("Hello, {}!", name).to_string(),
        success: true,
    })
}