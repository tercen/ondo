use super::rocks_trait::RocksTrait;
use crate::db::db_error::DbError::CfNotFound;
use crate::db::db_error::{DbError, DbResult};
use crate::db::entity::reference::requests::table_stored_requests::TableStoredIteratorRequests;
use crate::db::entity::reference::requests::table_stored_requests::TableStoredRequests;
use crate::db::entity::reference::table_reference::TableName;
use crate::db::entity::table::TableStored;
use crate::db::entity::table_value::TableValue;
use crate::db::server::rocks_db_accessor::DbReadLockGuardWrapper;
use crate::db::server::rocks_db_accessor::RocksDbAccessor;
use crate::db::server::source_sink::ondo_serializer::OndoSerializer;
use serde_json::Value;

impl TableStoredRequests for RocksDbAccessor {
    fn get_table_stored(&self, cf_name: &str, key: &TableName) -> DbResult<Option<TableStored>> {
        let guarded_db = self.guarded_db();
        let db = RocksDbAccessor::db_read_lock(&guarded_db)?;
        let cf = db.cf_handle(cf_name).ok_or(CfNotFound)?;
        let ondo_key = TableName::ondo_serialize(key)?;
        let answer = db
            .get_cf(cf, &ondo_key)
            .map_err(|err| DbError::RocksDbError(err))?;
        answer
            .map(|bytes| TableStored::ondo_deserialize(&bytes))
            .transpose()
    }
}

impl<'a> TableStoredIteratorRequests<'a> for DbReadLockGuardWrapper<'a> {
    fn all_values(
        &'a self,
        value_cf_name: &str,
    ) -> DbResult<Box<dyn Iterator<Item = DbResult<TableValue>> + 'a>> {
        let raw_all_iterator = self.guard.get_records_in_cf(value_cf_name)?;

        let all_iterator = raw_all_iterator.map(|result| {
            result.and_then(|(_, v)| Value::ondo_deserialize(&v)) // Flatten the nested Result
        });

        let ok_iterator = Box::new(all_iterator);
        Ok(ok_iterator)
    }
}
