use super::OndoSerializer;
use crate::db::db_error::DbError;
use crate::db::db_error::DbResult;
use bincode::{deserialize, serialize};
use serde_json::Value;

impl OndoSerializer<Value> for Value {
    fn ondo_serialize(&self) -> DbResult<Vec<u8>> {
        serialize(self).map_err(|_| DbError::SerializationError)
    }
    fn ondo_deserialize(bytes: &[u8]) -> DbResult<Value> {
        let serde_value = deserialize::<Value>(bytes).map_err(|_| DbError::SerializationError)?;
        Ok(serde_value)
    }
}
