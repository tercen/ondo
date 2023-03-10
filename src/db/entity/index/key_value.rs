//key_value.rs
pub type IndexValue = serde_json::Value;
pub type IndexKey = Vec<IndexValue>;

#[derive(Debug, PartialEq, Eq)]
pub struct KeyValue {
    key: IndexKey,
    value: IndexValue,
}

impl KeyValue{
    pub fn new(key: IndexKey, value: IndexValue) -> Self {
        Self { key: key, value: value }
    }
}
