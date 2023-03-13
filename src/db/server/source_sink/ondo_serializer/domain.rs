use super::OndoSerializer;

use crate::db::db_error::DbError;
use crate::db::db_error::DbResult;
use crate::db::entity::domain::DomainStored;
use crate::db::entity::reference::domain_reference::DomainReference;
use bincode::{deserialize, serialize};
use serde_json::{json, Value};

impl OndoSerializer<DomainStored> for DomainStored {
    fn ondo_serialize(&self) -> DbResult<Vec<u8>> {
        let serde_value = json!(self);
        serialize(&serde_value).map_err(|_| DbError::SerializationError)
    }
    fn ondo_deserialize(bytes: &[u8]) -> DbResult<DomainStored> {
        let serde_value = deserialize::<Value>(bytes).map_err(|_| DbError::SerializationError)?;
        let answer =
            serde_json::from_value(serde_value).map_err(|_| DbError::SerializationError)?;
        Ok(answer)
    }
}

impl OndoSerializer<DomainReference> for DomainReference {
    fn ondo_serialize(&self) -> DbResult<Vec<u8>> {
        let serde_value = json!(self);
        serialize(&serde_value).map_err(|_| DbError::SerializationError)
    }
    fn ondo_deserialize(bytes: &[u8]) -> DbResult<DomainReference> {
        let serde_value = deserialize::<Value>(bytes).map_err(|_| DbError::SerializationError)?;
        let answer =
            serde_json::from_value(serde_value).map_err(|_| DbError::SerializationError)?;
        Ok(answer)
    }
}
