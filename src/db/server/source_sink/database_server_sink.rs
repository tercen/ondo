use super::ondo_serializer::OndoSerializer;
use crate::db::db_error::DbError;
use crate::db::entity::database_server::DatabaseServerStored;
use crate::db::entity::reference::database_server_reference::DatabaseServerName;
use crate::db::entity::reference::effect::database_server_stored_effect::DatabaseServerStoredEffect;
use crate::db::server::rocks_db_accessor::RocksDbAccessor;

pub(super) fn apply_effect(
    ra: &RocksDbAccessor,
    effect: &DatabaseServerStoredEffect,
) -> Result<(), DbError> {
    let guarded_db = ra.guarded_db();
    let db = guarded_db.read().map_err(|_| DbError::CanNotLockDbMutex)?;
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
