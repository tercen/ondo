use super::ondo_serializer::OndoSerializer;
use crate::db::entity::OndoKey;
use crate::db::reference::IndexValueEffect;
use crate::db::server::lockable_db::transaction_maker::TransactionMaker;
use crate::db::DbError;

pub(super) fn apply_effect(
    ra: &TransactionMaker,
    effect: &IndexValueEffect,
) -> Result<(), DbError> {
    let db_guard = ra.read();
    let db = db_guard.inner();
    match effect {
        IndexValueEffect::Put(cf_name, key, index_value) => {
            let ondo_key = OndoKey::ondo_serialize(&key)?;
            let ondo_value = OndoKey::ondo_serialize(&index_value)?;
            let cf = db.cf_handle(&cf_name).ok_or(DbError::CfNotFound)?;
            db.put_cf(&cf, ondo_key, ondo_value)
                .map_err(|err| DbError::RocksDbError(err))
        }
        IndexValueEffect::Delete(cf_name, key) => {
            let ondo_key = OndoKey::ondo_serialize(&key)?;
            let cf = db.cf_handle(&cf_name).ok_or(DbError::CfNotFound)?;
            db.delete_cf(&cf, ondo_key)
                .map_err(|err| DbError::RocksDbError(err))
        }
    }
}
