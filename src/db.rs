use rusqlite::{params, Connection, Result as SqliteResult};
use std::fs;

use crate::error::Result;
use crate::models::Workspace;

pub struct CursorDB {
    pub workspace: Workspace,
}

impl CursorDB {
    pub fn new(workspace: Workspace) -> Self {
        Self { workspace }
    }

    fn create_table_if_not_exists(conn: &Connection) -> SqliteResult<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS ItemTable (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            params![],
        )?;
        Ok(())
    }

    fn get_db_connection(&self) -> Result<Connection> {
        let db_path = self.workspace.path.join("state.vscdb");
        if !db_path.exists() {
            let conn = Connection::open(&db_path)?;
            Self::create_table_if_not_exists(&conn)?;
            return Ok(conn);
        }

        // Check if the file is a valid SQLite database
        let is_valid = match fs::read(&db_path) {
            Ok(bytes) => bytes.starts_with(b"SQLite format 3"),
            Err(_) => false,
        };

        if !is_valid {
            // If not valid, delete and create a new one
            let _ = fs::remove_file(&db_path);
            let conn = Connection::open(&db_path)?;
            Self::create_table_if_not_exists(&conn)?;
            return Ok(conn);
        }

        let conn = Connection::open(&db_path)?;
        Self::create_table_if_not_exists(&conn)?;
        Ok(conn)
    }

    pub fn get_notepad_data(&self) -> Result<Option<String>> {
        let conn = self.get_db_connection()?;
        let mut stmt = conn.prepare("SELECT value FROM ItemTable WHERE key = 'notepad/state'")?;
        let data: Option<String> = stmt.query_row(params![], |row| row.get(0)).ok();
        Ok(data)
    }

    pub fn set_notepad_data(&self, json_str: &str) -> Result<()> {
        let conn = self.get_db_connection()?;
        conn.execute(
            "INSERT OR REPLACE INTO ItemTable (key, value) VALUES ('notepad/state', ?1)",
            params![json_str],
        )?;
        Ok(())
    }
} 