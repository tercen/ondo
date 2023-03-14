use super::ondo_serializer::OndoSerializer;
use crate::db::db_error::DbError;
use crate::db::entity::domain::DomainStored;
use crate::db::entity::reference::domain_reference::DomainName;
use crate::db::entity::reference::effect::domain_stored_effect::DomainStoredEffect;
use crate::db::server::rocks_db_accessor::RocksDbAccessor;

pub(super) fn apply_effect(
    ra: &RocksDbAccessor,
    effect: &DomainStoredEffect,
) -> Result<(), DbError> {
    let guarded_db = ra.guarded_db();
    let db = guarded_db.read().map_err(|_| DbError::CanNotLockDbMutex)?;
    match effect {
        DomainStoredEffect::Put(cf_name, key, domain_stored) => {
            let ondo_key = DomainName::ondo_serialize(&key)?;
            let ondo_value = DomainStored::ondo_serialize(&domain_stored)?;
            let cf = db.cf_handle(&cf_name).ok_or(DbError::CfNotFound)?;
            db.put_cf(&cf, ondo_key, ondo_value)
                .map_err(|err| DbError::RocksDbError(err))
        }
        DomainStoredEffect::Delete(cf_name, key) => {
            let ondo_key = DomainName::ondo_serialize(&key)?;
            let cf = db.cf_handle(&cf_name).ok_or(DbError::CfNotFound)?;
            db.delete_cf(&cf, ondo_key)
                .map_err(|err| DbError::RocksDbError(err))
        }
    }
}
