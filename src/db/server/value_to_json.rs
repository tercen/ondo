use serde_json::Value;

pub trait ValueToJson {
    fn json_to_value(json_string: &String) -> Self;
    fn value_to_json(&self) -> String;
}

impl ValueToJson for Value {
    fn json_to_value(json_string: &String) -> Self {
        serde_json::from_str(json_string).unwrap()
    }

    fn value_to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}