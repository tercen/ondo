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
    CanNotLockDbMutex,
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
            DbError::CanNotLockDbMutex => write!(f, "Can not lock db mutex"),
            DbError::Other => write!(f, "Other"),
        }
    }
}

pub(crate) type DbResult<T> = Result<T, DbError>;

impl From<DbError> for u32 {
    fn from(err: DbError) -> Self {
        match err {
            DbError::NotFound => 1,
            DbError::DatabaseNotInitialized => 2,
            DbError::DomainNotInitialized => 3,
            DbError::TableNotInitialized => 4,
            DbError::IndexNotInitialized => 5,
            DbError::AlreadyExists => 6,
            DbError::NotU64 => 7,
            DbError::CanNotLockDbMutex => 8,
            DbError::Other => 9,
        }
    }
}
