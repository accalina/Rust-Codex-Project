use axum::{
    routing::get,
    Router,
    extract::Extension,
};
use std::sync::Arc;

mod domain;
mod database;
mod repository;
mod service;
mod api;

#[tokio::main]
async fn main() {
    // Initialize database
    let db = database::Database::new().expect("Failed to initialize database");
    
    // Create repository
    let repository = repository::sqlite_repository::SqliteGreetingRepository::new(db);
    
    // Create our service implementation
    let service: Arc<dyn domain::GreetingService + Send + Sync> = 
        Arc::new(service::GreetingServiceImpl::new(repository));
    
    // Build our application with the service in its state
    let app = Router::new()
        .route("/", get(api::hello_world_handler))
        .route("/hello/{name}", get(api::hello_name_handler))
        .route("/history", get(api::history_handler))
        .layer(Extension(service));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8008").await.unwrap();
    println!("Listening on http://127.0.0.1:8008");
    axum::serve(listener, app).await.unwrap();
}
