use super::ondo_serializer::OndoSerializer;
use crate::db::entity::DatabaseServerStored;
use crate::db::reference::database_server_reference::DatabaseServerName;
use crate::db::reference::effect::database_server_stored_effect::DatabaseServerStoredEffect;

use crate::db::DbError;
use crate::db::server::lockable_db::transaction_or_db::TransactionOrDb;

pub(super) fn apply_effect<'a>(
    db: &TransactionOrDb<'a>,
    effect: &DatabaseServerStoredEffect,
) -> Result<(), DbError> {
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
