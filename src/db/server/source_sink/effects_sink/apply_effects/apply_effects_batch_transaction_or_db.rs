use tonic::Status;

use crate::db::{
    db_error::DbError, reference::{Effect, AccessEffect}, server::{lockable_db::transaction_or_db::TransactionOrDb, db_error_to_status::DbErrorToStatus},
};

pub(super) fn apply_effects_batch_transaction_or_db<'a>(
    transaction_or_db: &TransactionOrDb<'a>,
    effects: &[Effect],
) -> Result<(), DbError> {
    for effect in effects {
        println!("Effect: {:?}", effect);
        match effect {
            Effect::Access(access) => {
                apply_access_effect(&transaction_or_db, &access)
                    .map_err(|e| DbError::TantivyError(e.to_string()))?;
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}

pub(crate) fn apply_access_effect<'a>(
    db: &TransactionOrDb<'a>,
    access_effect: &AccessEffect,
) -> Result<(), Status> {
    match access_effect {
        AccessEffect::DatabaseServerStoredEffect(effect) => {
            super::super::super::database_server_sink::apply_effect(&db, effect).map_db_err_to_status()?;
        }
        AccessEffect::DomainStoredEffect(effect) => {
            super::super::super::domain_sink::apply_effect(&db, effect).map_db_err_to_status()?;
        }
        AccessEffect::TableStoredEffect(effect) => {
            super::super::super::table_sink::apply_effect(&db, effect).map_db_err_to_status()?;
        }
        AccessEffect::TableValueEffect(effect) => {
            super::super::super::table_value_sink::apply_effect(&db, effect).map_db_err_to_status()?;
        }
        AccessEffect::IndexValueEffect(effect) => {
            super::super::super::index_value_sink::apply_effect(&db, effect).map_db_err_to_status()?;
        }
        AccessEffect::ColumnValueEffect(effect) => {
            super::super::super::column_value_sink::apply_effect(&db, effect).map_db_err_to_status()?;
        }
    }
    Ok(())
}
