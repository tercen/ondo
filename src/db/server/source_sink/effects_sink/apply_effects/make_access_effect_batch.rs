use super::{
    make_column_value_effect_batch::make_column_value_effect_batch,
    make_database_server_stored_effect_batch::make_database_server_stored_effect_batch,
    make_domain_stored_effect_batch::make_domain_stored_effect_batch,
    make_index_value_effect_batch::make_index_value_effect_batch,
    make_table_stored_effect_batch::make_table_stored_effect_batch,
    make_table_value_effect_batch::make_table_value_effect_batch,
};
use crate::db::{db_error::DbError, reference::AccessEffect};
use rocksdb::{TransactionDB, WriteBatchWithTransaction};

pub(super) fn make_access_effect_batch<'a>(
    db: &TransactionDB,
    access_effect: &AccessEffect,
    batch: &mut WriteBatchWithTransaction<true>,
) -> Result<(), DbError> {
    match access_effect {
        AccessEffect::DatabaseServerStoredEffect(effect) => {
            make_database_server_stored_effect_batch(db, effect, batch)?;
        }
        AccessEffect::DomainStoredEffect(effect) => {
            make_domain_stored_effect_batch(db, effect, batch)?;
        }
        AccessEffect::TableStoredEffect(effect) => {
            make_table_stored_effect_batch(db, effect, batch)?;
        }
        AccessEffect::TableValueEffect(effect) => {
            make_table_value_effect_batch(db, effect, batch)?;
        }
        AccessEffect::IndexValueEffect(effect) => {
            make_index_value_effect_batch(db, effect, batch)?;
        }
        AccessEffect::ColumnValueEffect(effect) => {
            make_column_value_effect_batch(db, effect, batch)?;
        }
    }

    Ok(())
}
