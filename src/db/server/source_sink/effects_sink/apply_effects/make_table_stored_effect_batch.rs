use crate::db::{
    db_error::DbError,
    entity::TableStored,
    reference::{TableName, TableStoredEffect},
    server::source_sink::ondo_serializer::OndoSerializer,
};
use rocksdb::{TransactionDB, WriteBatchWithTransaction};

pub(super) fn make_table_stored_effect_batch<'a>(
    transaction_or_db: &TransactionDB,
    effect: &TableStoredEffect,
    batch: &mut WriteBatchWithTransaction<true>,
) -> Result<(), DbError> {
    match effect {
        TableStoredEffect::Put(cf_name, key, table_stored) => {
            let ondo_key = TableName::ondo_serialize(key)?;
            let ondo_value = TableStored::ondo_serialize(table_stored)?;
            let cf = transaction_or_db
                .cf_handle(cf_name)
                .ok_or(DbError::CfNotFound)?;

            batch.put_cf(&cf, ondo_key, ondo_value);
        }
        TableStoredEffect::Delete(cf_name, key) => {
            let ondo_key = TableName::ondo_serialize(key)?;
            let cf = transaction_or_db
                .cf_handle(cf_name)
                .ok_or(DbError::CfNotFound)?;

            batch.delete_cf(&cf, ondo_key);
        }
    }

    Ok(())
}
