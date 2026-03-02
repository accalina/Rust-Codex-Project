// This module handles SQLite-specific database operations
// It's focused solely on data access and persistence

use crate::database::{Database, GreetingRecord};
use rusqlite::Result as SqlResult;

pub struct SqliteGreetingRepository {
    db: Database,
}

impl SqliteGreetingRepository {
    pub fn new(db: Database) -> Self {
        SqliteGreetingRepository { db }
    }
    
    pub fn save_greeting(&self, name: &str, message: &str) -> SqlResult<()> {
        self.db.save_greeting(name, message)
    }
    
    pub fn get_greeting_history(&self, limit: u32) -> SqlResult<Vec<GreetingRecord>> {
        self.db.get_greeting_history(limit)
    }
}
