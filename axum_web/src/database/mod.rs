use rusqlite::{Connection, Result as SqlResult};
use std::sync::{Arc, Mutex};

pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn new() -> SqlResult<Self> {
        let conn = Connection::open("greetings.db")?;
        
        // Create the greetings table if it doesn't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS greetings (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                message TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;
        
        Ok(Database {
            conn: Arc::new(Mutex::new(conn)),
        })
    }
    
    pub fn save_greeting(&self, name: &str, message: &str) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO greetings (name, message) VALUES (?1, ?2)",
            [name, message],
        )?;
        Ok(())
    }
    
    pub fn get_greeting_history(&self, limit: u32) -> SqlResult<Vec<GreetingRecord>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT name, message, created_at FROM greetings ORDER BY created_at DESC LIMIT ?1"
        )?;
        
        let records = stmt.query_map([limit], |row| {
            Ok(GreetingRecord {
                name: row.get(0)?,
                message: row.get(1)?,
                created_at: row.get(2)?,
            })
        })?;
        
        let mut result = Vec::new();
        for record in records {
            result.push(record?);
        }
        
        Ok(result)
    }
}

pub struct GreetingRecord {
    pub name: String,
    pub message: String,
    pub created_at: String,
}