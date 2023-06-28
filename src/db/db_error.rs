//db_error.rs
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum DbError {
    Other(String),
    DatabaseNotInitialized,
    DomainNotInitialized,
    TableNotInitialized,
    IndexNotInitialized,
    AlreadyExists,
    NotFound,
    NotU64,
    CanNotLockDbMutex,
    SerializationError(String),
    CfNotFound,
    RocksDbError(rocksdb::Error),
    TantivyError(String),
    TransactionError(rocksdb::Error),
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DbError::Other(msg) => write!(f, "Error: {}!", &msg),
            DbError::DatabaseNotInitialized => write!(f, "DatabaseNotInitialized"),
            DbError::DomainNotInitialized => write!(f, "DomainNotInitialized"),
            DbError::TableNotInitialized => write!(f, "TableNotInitialized"),
            DbError::IndexNotInitialized => write!(f, "IndexNotInitialized"),
            DbError::AlreadyExists => write!(f, "AlreadyExists"),
            DbError::NotFound => write!(f, "NotFound"),
            DbError::NotU64 => write!(f, "Not u64"),
            DbError::CanNotLockDbMutex => write!(f, "Can not lock db mutex"),
            DbError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            DbError::CfNotFound => write!(f, "Column family not found"),
            DbError::RocksDbError(err) => write!(f, "RocksDbError: {}", err),
            DbError::TantivyError(msg) => write!(f, "TantivyError: {}", msg),
            DbError::TransactionError(err) => write!(f, "RocksDbError: {}", err),
        }
    }
}

pub(crate) type DbResult<T> = Result<T, DbError>;

impl From<DbError> for u32 {
    fn from(err: DbError) -> Self {
        match err {
            DbError::Other(_) => 0,
            DbError::NotFound => 1,
            DbError::DatabaseNotInitialized => 2,
            DbError::DomainNotInitialized => 3,
            DbError::TableNotInitialized => 4,
            DbError::IndexNotInitialized => 5,
            DbError::AlreadyExists => 6,
            DbError::NotU64 => 7,
            DbError::CanNotLockDbMutex => 8,
            DbError::SerializationError(_) => 9,
            DbError::CfNotFound => 10,
            DbError::RocksDbError(_) => 11,
            DbError::TantivyError(_) => 12,
            DbError::TransactionError(_) => 13,
        }
    }
}
