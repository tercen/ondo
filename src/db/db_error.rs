//db_error.rs
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum DbError {
    DatabaseNotInitialized,
    DomainNotInitialized,
    TableNotInitialized,
    IndexNotInitialized,
    AlreadyExists,
    NotFound,
    NotU64,
    Other,
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DbError::DatabaseNotInitialized => write!(f, "DatabaseNotInitialized"),
            DbError::DomainNotInitialized => write!(f, "DomainNotInitialized"),
            DbError::TableNotInitialized => write!(f, "TableNotInitialized"),
            DbError::IndexNotInitialized => write!(f, "IndexNotInitialized"),
            DbError::AlreadyExists => write!(f, "AlreadyExists"),
            DbError::NotFound => write!(f, "NotFound"),
            DbError::NotU64 => write!(f, "Not u64"),
            DbError::Other => write!(f, "Other"),
        }
    }
}

pub type DbResult<T> = Result<T, DbError>;
