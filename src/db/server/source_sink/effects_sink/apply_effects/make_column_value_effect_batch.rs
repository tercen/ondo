use crate::db::{
    db_error::DbError, entity::OndoKey, reference::ColumnValueEffect,
    server::source_sink::ondo_serializer::OndoSerializer,
};
use rocksdb::WriteBatchWithTransaction;
use serde_json::Value;

pub(super) fn make_column_value_effect_batch(
    db: &rocksdb::TransactionDB,
    effect: &ColumnValueEffect,
    batch: &mut WriteBatchWithTransaction<true>,
) -> Result<(), DbError> {
    match effect {
        ColumnValueEffect::Put(cf_name, key, value) => {
            let ondo_key = OndoKey::ondo_serialize(key)?;
            let ondo_value = Value::ondo_serialize(value)?;
            let cf = db.cf_handle(cf_name).ok_or(DbError::CfNotFound)?;

            batch.put_cf(&cf, ondo_key, ondo_value);
        }
        ColumnValueEffect::Delete(cf_name, key) => {
            let ondo_key = OndoKey::ondo_serialize(key)?;
            let cf = db.cf_handle(cf_name).ok_or(DbError::CfNotFound)?;

            batch.delete_cf(&cf, ondo_key);
        }
    }

    Ok(())
}
