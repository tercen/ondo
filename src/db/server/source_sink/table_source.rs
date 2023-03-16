use crate::db::db_error::DbError::CfNotFound;
use crate::db::db_error::{DbError, DbResult};
use crate::db::entity::table::TableStored;
use crate::db::entity::reference::table_reference::TableName;
use crate::db::entity::reference::requests::table_stored_requests::TableStoredRequests;
use crate::db::server::rocks_db_accessor::RocksDbAccessor;
use crate::db::server::source_sink::ondo_serializer::OndoSerializer;

impl TableStoredRequests for RocksDbAccessor {
    fn get_table_stored(
        &self,
        cf_name: &str,
        key: &TableName,
    ) -> DbResult<Option<TableStored>> {
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
    fn iter<'a>(&'a self, _value_cf_name: &str) -> crate::callback_iterator::CallbackIterator<'a, crate::db::entity::table_value::TableValue> {
        todo!();
    }
}
