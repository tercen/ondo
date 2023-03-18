use super::ondo_serializer::OndoSerializer;
use crate::db::db_error::DbError::CfNotFound;
use crate::db::db_error::{DbError, DbResult};
use crate::db::entity::reference::column_value_reference::ColumnValue;
use crate::db::entity::reference::requests::column_value_requests::ColumnValueRequests;
use crate::db::server::rocks_db_accessor::RocksDbAccessor;
use serde_json::Value;

impl ColumnValueRequests for RocksDbAccessor {
    fn get_column_value(&self, cf_name: &str, key: &Value) -> DbResult<Option<ColumnValue>> {
        let guarded_db = self.guarded_db();
        let db = RocksDbAccessor::db_read_lock(&guarded_db)?;
        let cf = db.cf_handle(cf_name).ok_or(CfNotFound)?;
        let ondo_key = Value::ondo_serialize(key)?;
        let answer = db
            .get_cf(cf, &ondo_key)
            .map_err(|err| DbError::RocksDbError(err))?;
        answer
            .map(|bytes| Value::ondo_deserialize(&bytes))
            .transpose()
    }
}
