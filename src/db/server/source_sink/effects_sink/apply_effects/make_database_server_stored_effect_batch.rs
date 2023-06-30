use crate::db::{
    db_error::DbError,
    entity::DatabaseServerStored,
    reference::{DatabaseServerName, DatabaseServerStoredEffect},
    server::source_sink::ondo_serializer::OndoSerializer,
};
use rocksdb::{TransactionDB, WriteBatchWithTransaction};

pub(super) fn make_database_server_stored_effect_batch(
    db: &TransactionDB,
    effect: &DatabaseServerStoredEffect,
    batch: &mut WriteBatchWithTransaction<true>,
) -> Result<(), DbError> {
    match effect {
        DatabaseServerStoredEffect::Put(cf_name, key, database_server_stored) => {
            let ondo_key = DatabaseServerName::ondo_serialize(key)?;
            let ondo_value = DatabaseServerStored::ondo_serialize(database_server_stored)?;
            let cf = db
                .cf_handle(cf_name)
                .ok_or(DbError::CfNotFound)?;

            batch.put_cf(&cf, ondo_key, ondo_value);
        }
        DatabaseServerStoredEffect::Delete(cf_name, key) => {
            let ondo_key = DatabaseServerName::ondo_serialize(key)?;
            let cf = db
                .cf_handle(cf_name)
                .ok_or(DbError::CfNotFound)?;

            batch.delete_cf(&cf, ondo_key);
        }
    }

    Ok(())
}
