use serde::{Deserialize, Serialize};
use serde_json::Value;

pub(crate) type OptionalOndoKey = Option<OndoKey>;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub(crate) struct OndoKey {
    pub values: Vec<Value>,
}

impl OndoKey {
    pub(crate) fn to_value(&self) -> serde_json::Value {
        serde_json::to_value(&self).unwrap()
    }

    pub(crate) fn from_value(value: &serde_json::Value) -> OndoKey {
        serde_json::from_value(value.clone()).unwrap()
    }
}
