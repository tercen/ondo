use crate::ondo_remote;
use ondo_remote::*;

impl Into<JsonMessage> for serde_json::Value {
    fn into(self) -> JsonMessage {
        JsonMessage {
            json: serde_json::to_string(&self).unwrap(),
        }
    }
}

impl Into<serde_json::Value> for JsonMessage {
    fn into(self) -> serde_json::Value {
        serde_json::from_str(&self.json).unwrap()
    }
}
