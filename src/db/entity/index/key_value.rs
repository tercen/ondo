
pub type Value = serde_json::Value;
pub type Key = Vec<Value>;

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
