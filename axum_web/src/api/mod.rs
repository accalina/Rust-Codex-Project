use axum::{
    extract::{Path, Query},
    Json,
};
use std::collections::HashMap;
use crate::domain::{GreetingService, UserResponse, GreetingHistoryResponse};

pub async fn hello_world_handler(
    service: axum::extract::Extension<std::sync::Arc<dyn GreetingService + Send + Sync>>
) -> Json<UserResponse> {
    Json(service.hello_world())
}

pub async fn hello_name_handler(
    Path(name): Path<String>,
    service: axum::extract::Extension<std::sync::Arc<dyn GreetingService + Send + Sync>>
) -> Json<UserResponse> {
    Json(service.hello_name(name))
}

pub async fn history_handler(
    Query(params): Query<HashMap<String, String>>,
    service: axum::extract::Extension<std::sync::Arc<dyn GreetingService + Send + Sync>>
) -> Json<GreetingHistoryResponse> {
    let limit: u32 = params.get("limit")
        .and_then(|s| s.parse().ok())
        .unwrap_or(10); // Default to 10 if not specified or invalid
    
    Json(service.get_greeting_history(limit))
}
