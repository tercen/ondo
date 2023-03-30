//key_value.rs
use crate::db::entity::OndoKey;
pub(crate) type IndexValue = serde_json::Value;
pub(crate) type IndexKey = OndoKey;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct KeyValue {
    pub key: IndexKey,
    pub value: IndexValue,
}

impl KeyValue {
    pub fn new(key: IndexKey, value: IndexValue) -> Self {
        Self {
            key: key,
            value: value,
        }
    }
}
