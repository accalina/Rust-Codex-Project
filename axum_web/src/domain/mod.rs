use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct UserResponse {
    pub message: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct GreetingRecord {
    pub name: String,
    pub message: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct GreetingHistoryResponse {
    pub greetings: Vec<GreetingRecord>,
    pub success: bool,
}

pub trait GreetingService {
    fn hello_world(&self) -> UserResponse;
    fn hello_name(&self, name: String) -> UserResponse;
    fn get_greeting_history(&self, limit: u32) -> GreetingHistoryResponse;
}
