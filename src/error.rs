use std::error::Error as StdError;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum CursorError {
    Io(io::Error),
    Database(rusqlite::Error),
    Json(serde_json::Error),
    Config(String),
}

impl fmt::Display for CursorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CursorError::Io(err) => write!(f, "I/O error: {}", err),
            CursorError::Database(err) => write!(f, "Database error: {}", err),
            CursorError::Json(err) => write!(f, "JSON error: {}", err),
            CursorError::Config(err) => write!(f, "Configuration error: {}", err),
        }
    }
}

impl StdError for CursorError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            CursorError::Io(err) => Some(err),
            CursorError::Database(err) => Some(err),
            CursorError::Json(err) => Some(err),
            CursorError::Config(_) => None,
        }
    }
}

impl From<io::Error> for CursorError {
    fn from(err: io::Error) -> Self {
        CursorError::Io(err)
    }
}

impl From<rusqlite::Error> for CursorError {
    fn from(err: rusqlite::Error) -> Self {
        CursorError::Database(err)
    }
}

impl From<serde_json::Error> for CursorError {
    fn from(err: serde_json::Error) -> Self {
        CursorError::Json(err)
    }
}

impl From<&str> for CursorError {
    fn from(err: &str) -> Self {
        CursorError::Config(err.to_string())
    }
}

impl From<String> for CursorError {
    fn from(err: String) -> Self {
        CursorError::Config(err)
    }
}

pub type Result<T> = std::result::Result<T, CursorError>; 