use super::OndoSerializer;
use crate::db::DbError;
use crate::db::DbResult;
use rmp_serde::{from_slice, to_vec};
use serde_json::Value;

impl OndoSerializer<Value> for Value {
    fn ondo_serialize(&self) -> DbResult<Vec<u8>> {
        to_vec(self).map_err(|e| DbError::SerializationError(e.to_string()))
    }
    fn ondo_deserialize(bytes: &[u8]) -> DbResult<Value> {
        let serde_value =
            from_slice::<Value>(bytes).map_err(|e| DbError::SerializationError(e.to_string()))?;
        Ok(serde_value)
    }
}
