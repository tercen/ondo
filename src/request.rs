use crate::types::*;

pub enum Request {
    Get(DomainName, TableName, Key),
}

pub struct RequestHandlers {
    pub get: fn(DomainName, TableName, Key) -> Value,
}