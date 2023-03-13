use crate::db::db_error::DbResult;

pub(super) mod database_server;
pub(super) mod domain;
pub(super) mod index;
pub(super) mod index_value;
pub(super) mod serde_value;
pub(super) mod string;
pub(super) mod table;
pub(super) mod table_value;
pub(super) mod vec_serde_value;
pub(super) mod column_value;
pub(super) mod unit;

pub(super) trait OndoSerializer<T> {
    fn ondo_serialize(&self) -> DbResult<Vec<u8>>;
    fn ondo_deserialize(bytes: &[u8]) -> DbResult<T>;
}
