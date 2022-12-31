#[derive(Debug, PartialEq)]
pub enum DbError {
    // NotFound,
    DatabaseNotInitialized,
    Other,
}

pub type DbResult<T> = Result<T, DbError>;
