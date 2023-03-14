use super::OndoSerializer;

use crate::db::db_error::DbError;
use crate::db::db_error::DbResult;
use crate::db::entity::index::IndexStored;
use crate::db::entity::reference::index_reference::IndexReference;
use rmp_serde::{from_slice, to_vec};
use serde_json::{json, Value};

impl OndoSerializer<IndexStored> for IndexStored {
    fn ondo_serialize(&self) -> DbResult<Vec<u8>> {
        let serde_value = json!(self);
        to_vec(&serde_value).map_err(|e| DbError::SerializationError(e.to_string()))
    }
    fn ondo_deserialize(bytes: &[u8]) -> DbResult<IndexStored> {
        let serde_value =
            from_slice::<Value>(bytes).map_err(|e| DbError::SerializationError(e.to_string()))?;
        let answer = serde_json::from_value(serde_value)
            .map_err(|e| DbError::SerializationError(e.to_string()))?;
        Ok(answer)
    }
}

impl OndoSerializer<IndexReference> for IndexReference {
    fn ondo_serialize(&self) -> DbResult<Vec<u8>> {
        let serde_value = json!(self);
        to_vec(&serde_value).map_err(|e| DbError::SerializationError(e.to_string()))
    }
    fn ondo_deserialize(bytes: &[u8]) -> DbResult<IndexReference> {
        let serde_value =
            from_slice::<Value>(bytes).map_err(|e| DbError::SerializationError(e.to_string()))?;
        let answer = serde_json::from_value(serde_value)
            .map_err(|e| DbError::SerializationError(e.to_string()))?;
        Ok(answer)
    }
}
