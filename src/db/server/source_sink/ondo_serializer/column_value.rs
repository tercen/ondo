use crate::db::entity::reference::column_value_reference::ColumnValueReference;
use super::OndoSerializer;
use crate::db::db_error::DbError;
use crate::db::db_error::DbResult;
use bincode::{deserialize, serialize};
use serde_json::{json, Value};

impl OndoSerializer<ColumnValueReference> for ColumnValueReference {
    fn ondo_serialize(&self) -> DbResult<Vec<u8>> {
        let serde_value = json!(self);
        serialize(&serde_value).map_err(|_| DbError::SerializationError)
    }
    fn ondo_deserialize(bytes: &[u8]) -> DbResult<ColumnValueReference> {
        let serde_value = deserialize::<Value>(bytes).map_err(|_| DbError::SerializationError)?;
        let answer =
            serde_json::from_value(serde_value).map_err(|_| DbError::SerializationError)?;
        Ok(answer)
    }
}
