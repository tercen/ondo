use super::OndoSerializer;

use crate::db::entity::DatabaseServerStored;
use crate::db::reference::database_server_reference::DatabaseServerReference;
use crate::db::DbError;
use crate::db::DbResult;
use serde_json::{json, Value};
// use serde::{Serialize, Deserialize};
use rmp_serde::{from_slice, to_vec};

impl OndoSerializer<DatabaseServerStored> for DatabaseServerStored {
    fn ondo_serialize(&self) -> DbResult<Vec<u8>> {
        let serde_value = json!(self);
        to_vec(&serde_value).map_err(|e| DbError::SerializationError(e.to_string()))
    }
    fn ondo_deserialize(bytes: &[u8]) -> DbResult<DatabaseServerStored> {
        let serde_value =
            from_slice::<Value>(bytes).map_err(|e| DbError::SerializationError(e.to_string()))?;
        let answer = serde_json::from_value(serde_value)
            .map_err(|e| DbError::SerializationError(e.to_string()))?;
        Ok(answer)
    }
}

impl OndoSerializer<DatabaseServerReference> for DatabaseServerReference {
    fn ondo_serialize(&self) -> DbResult<Vec<u8>> {
        let serde_value = json!(self);
        to_vec(&serde_value).map_err(|e| DbError::SerializationError(e.to_string()))
    }
    fn ondo_deserialize(bytes: &[u8]) -> DbResult<DatabaseServerReference> {
        let serde_value =
            from_slice::<Value>(bytes).map_err(|e| DbError::SerializationError(e.to_string()))?;
        let answer = serde_json::from_value(serde_value)
            .map_err(|e| DbError::SerializationError(e.to_string()))?;
        Ok(answer)
    }
}
