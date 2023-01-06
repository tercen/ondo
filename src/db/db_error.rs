use std::fmt;

#[derive(Debug, PartialEq)]
pub enum DbError {
    // NotFound,
    DatabaseNotInitialized,
    Other,
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DbError::DatabaseNotInitialized => write!(f, "DatabaseNotInitialized"),
            DbError::Other => write!(f, "Other"),
        }
    }
}

pub type DbResult<T> = Result<T, DbError>;
