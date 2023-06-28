use super::ondo_serializer::OndoSerializer;
use crate::db::db_error::{DbError, DbResult};
use crate::db::entity::ondo_key::OndoKey;
use crate::db::reference::requests::ColumnValueRequests;
use crate::db::reference::ColumnValue;

use crate::db::DbError::CfNotFound;
use crate::db::server::lockable_db::transaction_or_db::TransactionOrDb;
use serde_json::Value;

impl<'a> ColumnValueRequests for TransactionOrDb<'a> {
    fn get_column_value(&self, cf_name: &str, key: &OndoKey) -> DbResult<Option<ColumnValue>> {
        let db = self;
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
