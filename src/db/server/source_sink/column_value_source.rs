use super::ondo_serializer::OndoSerializer;
use crate::db::db_error::{DbError, DbResult};
use crate::db::entity::ondo_key::OndoKey;
use crate::db::reference::requests::ColumnValueRequests;
use crate::db::reference::ColumnValue;
use crate::db::server::lockable_db::LockableDb;
use crate::db::DbError::CfNotFound;
use serde_json::Value;

impl ColumnValueRequests for LockableDb {
    fn get_column_value(&self, cf_name: &str, key: &OndoKey) -> DbResult<Option<ColumnValue>> {
        let db = self.read();
        let cf = db.cf_handle(cf_name).ok_or(CfNotFound)?;
        let ondo_key = OndoKey::ondo_serialize(key)?;
        let answer = db
            .get_cf(cf, &ondo_key)
            .map_err(|err| DbError::RocksDbError(err))?;
        answer
            .map(|bytes| Value::ondo_deserialize(&bytes))
            .transpose()
    }
}
