use super::ondo_serializer::OndoSerializer;
use crate::db::entity::DatabaseServerStored;
use crate::db::reference::database_server_reference::DatabaseServerName;
use crate::db::reference::effect::database_server_stored_effect::DatabaseServerStoredEffect;
use crate::db::server::lockable_db::transaction_maker::TransactionMaker;
use crate::db::DbError;

pub(super) fn apply_effect(
    ra: &TransactionMaker,
    effect: &DatabaseServerStoredEffect,
) -> Result<(), DbError> {
    let db = ra.read();
    match effect {
        DatabaseServerStoredEffect::Put(cf_name, key, database_server_stored) => {
            let ondo_key = DatabaseServerName::ondo_serialize(&key)?;
            let ondo_value = DatabaseServerStored::ondo_serialize(&database_server_stored)?;
            let cf = db.cf_handle(&cf_name).ok_or(DbError::CfNotFound)?;
            db.put_cf(&cf, ondo_key, ondo_value)
                .map_err(|err| DbError::RocksDbError(err))
        }
        DatabaseServerStoredEffect::Delete(cf_name, key) => {
            let ondo_key = DatabaseServerName::ondo_serialize(&key)?;
            let cf = db.cf_handle(&cf_name).ok_or(DbError::CfNotFound)?;
            db.delete_cf(&cf, ondo_key)
                .map_err(|err| DbError::RocksDbError(err))
        }
    }
}
