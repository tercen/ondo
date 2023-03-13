use super::OndoSerializer;

use crate::db::db_error::DbError;
use crate::db::db_error::DbResult;
use crate::db::entity::reference::TableReference;
use crate::db::entity::table::TableStored;
use bincode::{deserialize, serialize};
use serde_json::{json, Value};

impl OndoSerializer<TableStored> for String {
    fn ondo_serialize(&self) -> DbResult<Vec<u8>> {
        let serde_value = json!(self);
        serialize(&serde_value).map_err(|_| DbError::SerializationError)
    }
    fn ondo_deserialize(bytes: &[u8]) -> DbResult<TableStored> {
        let serde_value = deserialize::<Value>(bytes).map_err(|_| DbError::SerializationError)?;
        let answer =
            serde_json::from_value(serde_value).map_err(|_| DbError::SerializationError)?;
        Ok(answer)
    }
}

impl OndoSerializer<TableReference> for TableReference {
    fn ondo_serialize(&self) -> DbResult<Vec<u8>> {
        let serde_value = json!(self);
        serialize(&serde_value).map_err(|_| DbError::SerializationError)
    }
    fn ondo_deserialize(bytes: &[u8]) -> DbResult<TableReference> {
        let serde_value = deserialize::<Value>(bytes).map_err(|_| DbError::SerializationError)?;
        let answer =
            serde_json::from_value(serde_value).map_err(|_| DbError::SerializationError)?;
        Ok(answer)
    }
}
