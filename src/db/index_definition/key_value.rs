use crate::db::names::*;

#[derive(Debug, PartialEq, Eq)]
pub struct KeyValue {
    key: Key,
    value: Value,
}

impl KeyValue{
    pub fn new(key: Key, value: Value) -> Self {
        Self { key: key, value: value }
    }
}
