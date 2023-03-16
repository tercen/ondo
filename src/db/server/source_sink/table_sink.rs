use super::ondo_serializer::OndoSerializer;
use crate::db::db_error::DbError;
use crate::db::entity::table::TableStored;
use crate::db::entity::reference::table_reference::TableName;
use crate::db::entity::reference::effect::table_stored_effect::TableStoredEffect;
use crate::db::server::rocks_db_accessor::RocksDbAccessor;

pub(super) fn apply_effect(
    ra: &RocksDbAccessor,
    effect: &TableStoredEffect,
) -> Result<(), DbError> {
    let guarded_db = ra.guarded_db();
    let db = RocksDbAccessor::db_read_lock(&guarded_db)?;
    match effect {
        TableStoredEffect::Put(cf_name, key, table_stored) => {
            let ondo_key = TableName::ondo_serialize(&key)?;
            let ondo_value = TableStored::ondo_serialize(&table_stored)?;
            let cf = db.cf_handle(&cf_name).ok_or(DbError::CfNotFound)?;
            db.put_cf(&cf, ondo_key, ondo_value)
                .map_err(|err| DbError::RocksDbError(err))
        }
        TableStoredEffect::Delete(cf_name, key) => {
            let ondo_key = TableName::ondo_serialize(&key)?;
            let cf = db.cf_handle(&cf_name).ok_or(DbError::CfNotFound)?;
            db.delete_cf(&cf, ondo_key)
                .map_err(|err| DbError::RocksDbError(err))
        }
    }
}
