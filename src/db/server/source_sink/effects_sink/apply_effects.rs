use crate::db::reference::effect::{AccessEffect, Effect, Effects, MetaEffect};
use crate::db::server::db_error_to_status::DbErrorToStatus;
use crate::db::server::lockable_db::transaction_or_db::TransactionOrDb;
use crate::db::DbError;
use crate::ondo_remote::EmptyMessage;
use rocksdb::TransactionDB;
use tonic::{Response, Status};

mod optimize_delete_cf_effects;
mod split_effects;

pub(crate) fn apply_meta_effect(
    db: &mut TransactionDB,
    meta_effect: &MetaEffect,
) -> Result<(), Status> {
    let cf_opts = rocksdb::Options::default();
    match meta_effect {
        MetaEffect::CreateCf(cf_name) => {
            db.create_cf(cf_name, &cf_opts)
                .map_err(|err| DbError::RocksDbError(err))
                .map_db_err_to_status()?;
        }
        MetaEffect::DeleteCf(cf_name) => {
            db.drop_cf(cf_name)
                .map_err(|err| DbError::RocksDbError(err))
                .map_db_err_to_status()?;
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
            super::super::database_server_sink::apply_effect(&db, effect).map_db_err_to_status()?;
        }
        AccessEffect::DomainStoredEffect(effect) => {
            super::super::domain_sink::apply_effect(&db, effect).map_db_err_to_status()?;
        }
        AccessEffect::TableStoredEffect(effect) => {
            super::super::table_sink::apply_effect(&db, effect).map_db_err_to_status()?;
        }
        AccessEffect::TableValueEffect(effect) => {
            super::super::table_value_sink::apply_effect(&db, effect).map_db_err_to_status()?;
        }
        AccessEffect::IndexValueEffect(effect) => {
            super::super::index_value_sink::apply_effect(&db, effect).map_db_err_to_status()?;
        }
        AccessEffect::ColumnValueEffect(effect) => {
            super::super::column_value_sink::apply_effect(&db, effect).map_db_err_to_status()?;
        }
    }
    Ok(())
}

pub(crate) fn apply_effects<'a>(
    db: &TransactionOrDb<'a>,
    effects: Effects,
) -> Result<Response<EmptyMessage>, Status> {
    let (meta_effects, access_effects) = split_effects::split_effects(effects);

    if !meta_effects.is_empty() {
        return Err(Status::invalid_argument("Meta effects are not allowed"));
    }

    for effect in access_effects {
        println!("Effect: {:?}", effect);
        match effect {
            Effect::Access(access) => apply_access_effect(db, &access)?,
            _ => unreachable!(),
        }
    }

    Ok(Response::new(EmptyMessage {}))
}
pub(crate) fn apply_all_effects(
    db: &mut TransactionDB,
    effects: Effects,
) -> Result<Response<EmptyMessage>, Status> {
    let optimized_effects = optimize_delete_cf_effects::optimize_delete_cf_effects(effects);
    let (meta_effects, access_effects) = split_effects::split_effects(optimized_effects);

    for effect in meta_effects {
        println!("Effect: {:?}", effect);
        match effect {
            Effect::Meta(meta) => apply_meta_effect(db, &meta)?,
            _ => unreachable!(),
        }
    }

    let transaction_or_db = TransactionOrDb::Db(db);

    for effect in access_effects {
        println!("Effect: {:?}", effect);
        match effect {
            Effect::Access(access) => {
                apply_access_effect(&transaction_or_db, &access)?;
            }
            _ => unreachable!(),
        }
    }

    Ok(Response::new(EmptyMessage {}))
}