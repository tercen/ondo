use super::OndoSerializer;
use crate::db::db_error::DbError;
use crate::db::db_error::DbResult;
use bincode::serialize;
use serde_json::json;

impl OndoSerializer<()> for () {
    fn ondo_serialize(&self) -> DbResult<Vec<u8>> {
        let serde_value = json!(null);
        serialize(&serde_value).map_err(|_| DbError::SerializationError)
    }
    fn ondo_deserialize(_: &[u8]) -> DbResult<()> {
        Ok(())
    }
}
