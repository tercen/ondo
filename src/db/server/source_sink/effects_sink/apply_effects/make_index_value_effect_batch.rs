use crate::db::{
    db_error::DbError, entity::OndoKey, reference::IndexValueEffect,
    server::source_sink::ondo_serializer::OndoSerializer,
};
use rocksdb::{TransactionDB, WriteBatchWithTransaction};

pub(super) fn make_index_value_effect_batch<'a>(
    db: &TransactionDB,
    effect: &IndexValueEffect,
    batch: &mut WriteBatchWithTransaction<true>,
) -> Result<(), DbError> {
    match effect {
        IndexValueEffect::Put(cf_name, key, index_value) => {
            let ondo_key = OndoKey::ondo_serialize(key)?;
            let ondo_value = OndoKey::ondo_serialize(index_value)?;
            let cf = db.cf_handle(cf_name).ok_or(DbError::CfNotFound)?;

            batch.put_cf(&cf, ondo_key, ondo_value);
        }
        IndexValueEffect::Delete(cf_name, key) => {
            let ondo_key = OndoKey::ondo_serialize(key)?;
            let cf = db.cf_handle(cf_name).ok_or(DbError::CfNotFound)?;

            batch.delete_cf(&cf, ondo_key);
        }
    }

    Ok(())
}
