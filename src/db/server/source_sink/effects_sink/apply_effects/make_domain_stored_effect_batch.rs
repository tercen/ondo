use crate::db::{
    db_error::DbError,
    entity::DomainStored,
    reference::{DomainName, DomainStoredEffect},
    server::{
        
        source_sink::ondo_serializer::OndoSerializer,
    },
};
use rocksdb::{WriteBatchWithTransaction, TransactionDB};

pub(super) fn make_domain_stored_effect_batch<'a>(
    db: &TransactionDB,
    effect: &DomainStoredEffect,
    batch: &mut WriteBatchWithTransaction<true>,
) -> Result<(), DbError> {
    match effect {
        DomainStoredEffect::Put(cf_name, key, domain_stored) => {
            let ondo_key = DomainName::ondo_serialize(key)?;
            let ondo_value = DomainStored::ondo_serialize(domain_stored)?;
            let cf = db
                .cf_handle(cf_name)
                .ok_or(DbError::CfNotFound)?;

            batch.put_cf(&cf, ondo_key, ondo_value);
        }
        DomainStoredEffect::Delete(cf_name, key) => {
            let ondo_key = DomainName::ondo_serialize(key)?;
            let cf = db
                .cf_handle(cf_name)
                .ok_or(DbError::CfNotFound)?;

            batch.delete_cf(&cf, ondo_key);
        }
    }

    Ok(())
}
