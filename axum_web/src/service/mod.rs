use crate::domain::{GreetingService, UserResponse, GreetingHistoryResponse};
use crate::repository::sqlite_repository::SqliteGreetingRepository;

pub struct GreetingServiceImpl {
    repository: SqliteGreetingRepository,
}

impl GreetingServiceImpl {
    pub fn new(repository: SqliteGreetingRepository) -> Self {
        GreetingServiceImpl { repository }
    }
}

impl GreetingService for GreetingServiceImpl {
    fn hello_world(&self) -> UserResponse {
        let message = "Hello, world!".to_string();
        
        // Save to database through repository
        if let Err(e) = self.repository.save_greeting("world", &message) {
            eprintln!("Failed to save greeting to database: {}", e);
        }
        
        UserResponse {
            message,
            success: true,
        }
    }

    fn hello_name(&self, name: String) -> UserResponse {
        let message = format!("Hello, {}!", name);
        
        // Save to database through repository
        if let Err(e) = self.repository.save_greeting(&name, &message) {
            eprintln!("Failed to save greeting to database: {}", e);
        }
        
        UserResponse {
            message,
            success: true,
        }
    }
    
    fn get_greeting_history(&self, limit: u32) -> GreetingHistoryResponse {
        match self.repository.get_greeting_history(limit) {
            Ok(records) => {
                let greetings = records.into_iter().map(|record| {
                    crate::domain::GreetingRecord {
                        name: record.name,
                        message: record.message,
                        created_at: record.created_at,
                    }
                }).collect();
                
                GreetingHistoryResponse {
                    greetings,
                    success: true,
                }
            },
            Err(e) => {
                eprintln!("Failed to retrieve greeting history: {}", e);
                GreetingHistoryResponse {
                    greetings: vec![],
                    success: false,
                }
            }
        }
    }
}
