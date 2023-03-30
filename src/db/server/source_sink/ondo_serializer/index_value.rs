use super::OndoSerializer;
use crate::db::DbError;
use crate::db::DbResult;
use crate::db::reference::IndexValueReference;
use rmp_serde::{from_slice, to_vec};
use serde_json::{json, Value};

impl OndoSerializer<IndexValueReference> for IndexValueReference {
    fn ondo_serialize(&self) -> DbResult<Vec<u8>> {
        let serde_value = json!(self);
        to_vec(&serde_value).map_err(|e| DbError::SerializationError(e.to_string()))
    }
    fn ondo_deserialize(bytes: &[u8]) -> DbResult<IndexValueReference> {
        let serde_value =
            from_slice::<Value>(bytes).map_err(|e| DbError::SerializationError(e.to_string()))?;
        let answer = serde_json::from_value(serde_value)
            .map_err(|e| DbError::SerializationError(e.to_string()))?;
        Ok(answer)
    }
}
