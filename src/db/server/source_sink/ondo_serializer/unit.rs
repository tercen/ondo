use super::OndoSerializer;
use crate::db::DbError;
use crate::db::DbResult;
use rmp_serde::to_vec;
use serde_json::json;

impl OndoSerializer<()> for () {
    fn ondo_serialize(&self) -> DbResult<Vec<u8>> {
        let serde_value = json!(null);
        to_vec(&serde_value).map_err(|e| DbError::SerializationError(e.to_string()))
    }
    fn ondo_deserialize(_: &[u8]) -> DbResult<()> {
        Ok(())
    }
}
