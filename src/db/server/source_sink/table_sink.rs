use super::ondo_serializer::OndoSerializer;
use crate::db::entity::TableStored;
use crate::db::reference::effect::TableStoredEffect;
use crate::db::reference::TableName;

use crate::db::DbError;
use crate::db::server::lockable_db::transaction_or_db::TransactionOrDb;

pub(super) fn apply_effect(
    transaction_or_dn: &TransactionOrDb,
    effect: &TableStoredEffect,
) -> Result<(), DbError> {
    match effect {
        TableStoredEffect::Put(cf_name, key, table_stored) => {
            let ondo_key = TableName::ondo_serialize(&key)?;
            let ondo_value = TableStored::ondo_serialize(&table_stored)?;
            let cf = transaction_or_dn.cf_handle(&cf_name).ok_or(DbError::CfNotFound)?;
            transaction_or_dn.put_cf(&cf, ondo_key, ondo_value)
                .map_err(|err| DbError::RocksDbError(err))
        }
        TableStoredEffect::Delete(cf_name, key) => {
            let ondo_key = TableName::ondo_serialize(&key)?;
            let cf = transaction_or_dn.cf_handle(&cf_name).ok_or(DbError::CfNotFound)?;
            transaction_or_dn.delete_cf(&cf, ondo_key)
                .map_err(|err| DbError::RocksDbError(err))
        }
    }
}
