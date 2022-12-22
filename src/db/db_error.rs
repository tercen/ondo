pub enum DbError {
    NotFound,
    Other,
}

pub type DbResult<T> = Result<T, DbError>;
