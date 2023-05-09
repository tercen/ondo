use super::ondo_serializer::OndoSerializer;
use crate::db::entity::TableStored;
use crate::db::reference::effect::TableStoredEffect;
use crate::db::reference::TableName;
use crate::db::server::lockable_db::transaction_maker::TransactionMaker;
use crate::db::DbError;

pub(super) fn apply_effect(
    ra: &TransactionMaker,
    effect: &TableStoredEffect,
) -> Result<(), DbError> {
    let db = ra.read();
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
