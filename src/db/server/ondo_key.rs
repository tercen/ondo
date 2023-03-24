use crate::db::entity::ondo_key::*;
use crate::ondo_remote;
use ondo_remote::*;

impl<'a> Into<OndoKey> for &'a OndoKeyMessage {
    fn into(self) -> OndoKey {
        let values = self
            .json_keys
            .iter()
            .map(|json_key| serde_json::from_str(json_key).unwrap())
            .collect();
        OndoKey { values }
    }
}

impl Into<OndoKeyMessage> for OndoKey {
    fn into(self) -> OndoKeyMessage {
        let json_keys = self
            .values
            .iter()
            .map(|value| serde_json::to_string(value).unwrap())
            .collect();
        OndoKeyMessage { json_keys }
    }
}
impl<'a> Into<OptionalOndoKey> for &'a OptionalOndoKeyMessage {
    fn into(self) -> OptionalOndoKey {
        let r_ondo_key = self.ondo_key.as_ref();
        r_ondo_key.map(|ondo_key| ondo_key.into())
    }
}

impl Into<OptionalOndoKeyMessage> for OptionalOndoKey {
    fn into(self) -> OptionalOndoKeyMessage {
        let ondo_key = self.map(|ondo_key| ondo_key.into());
        OptionalOndoKeyMessage { ondo_key }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ondo_key_message_into_ondo_key() {
        let message = OndoKeyMessage {
            json_keys: vec![r#"{"key":"value"}"#.to_string()],
        };
        let key: OndoKey = (&message).into();
        assert_eq!(key.values.len(), 1);
        assert_eq!(key.values[0]["key"], "value");
    }

    #[test]
    fn test_ondo_key_into_ondo_key_message() {
        let key = OndoKey {
            values: vec![serde_json::json!({"key":"value"})],
        };
        let message: OndoKeyMessage = key.into();
        assert_eq!(message.json_keys.len(), 1);
        assert_eq!(message.json_keys[0], r#"{"key":"value"}"#);
    }

    #[test]
    fn test_optional_ondo_key_message_into_optional_ondo_key() {
        let message = OptionalOndoKeyMessage {
            ondo_key: Some(OndoKeyMessage {
                json_keys: vec![r#"{"key":"value"}"#.to_string()],
            }),
        };
        let optional_key: OptionalOndoKey = (&message).into();
        assert_eq!(
            optional_key,
            Some(OndoKey {
                values: vec![serde_json::json!({"key":"value"})],
            })
        );
    }

    #[test]
    fn test_optional_ondo_key_into_optional_ondo_key_message() {
        let optional_key = Some(OndoKey {
            values: vec![serde_json::json!({"key":"value"})],
        });
        let message: OptionalOndoKeyMessage = optional_key.into();
        assert_eq!(message.clone().ondo_key.unwrap().json_keys.len(), 1);
        assert_eq!(message.ondo_key.unwrap().json_keys[0], r#"{"key":"value"}"#);
    }
}
