use crate::db::{
    db_error::DbError, entity::OndoKey, reference::TableValueEffect,
    server::source_sink::ondo_serializer::OndoSerializer,
};
use rocksdb::{TransactionDB, WriteBatchWithTransaction};
use serde_json::Value;

pub(super) fn make_table_value_effect_batch<'a>(
    transaction_or_db: &TransactionDB,
    effect: &TableValueEffect,
    batch: &mut WriteBatchWithTransaction<true>,
) -> Result<(), DbError> {
    match effect {
        TableValueEffect::Put(cf_name, ondo_key, value) => {
            let serialized_ondo_key = OndoKey::ondo_serialize(ondo_key)?;
            let ondo_value = Value::ondo_serialize(value)?;
            let cf = transaction_or_db
                .cf_handle(cf_name)
                .ok_or(DbError::CfNotFound)?;

            batch.put_cf(&cf, serialized_ondo_key, ondo_value);
        }
        TableValueEffect::Delete(cf_name, ondo_key) => {
            let serialized_ondo_key = OndoKey::ondo_serialize(ondo_key)?;
            let cf = transaction_or_db
                .cf_handle(cf_name)
                .ok_or(DbError::CfNotFound)?;

            batch.delete_cf(&cf, serialized_ondo_key);
        }
    }

    Ok(())
}
