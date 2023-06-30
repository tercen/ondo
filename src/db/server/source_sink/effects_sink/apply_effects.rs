use crate::db::reference::effect::{Effect, Effects, MetaEffect};
use crate::db::server::db_error_to_status::DbErrorToStatus;
use crate::db::server::lockable_db::transaction_or_db::TransactionOrDb;
use crate::db::DbError;
use crate::ondo_remote::EmptyMessage;
use rocksdb::TransactionDB;
use tonic::{Response, Status};

mod apply_effects_batch;
mod apply_effects_batch_db;
mod apply_effects_batch_transaction_or_db;
mod make_access_effect_batch;
mod make_column_value_effect_batch;
mod make_database_server_stored_effect_batch;
mod make_domain_stored_effect_batch;
mod make_index_value_effect_batch;
mod make_table_stored_effect_batch;
mod make_table_value_effect_batch;
mod optimize_delete_cf_effects;
mod split_effects;

pub(crate) fn apply_effects<'a>(
    transaction_or_db: &TransactionOrDb<'a>,
    effects: Effects,
) -> Result<Response<EmptyMessage>, Status> {
    let (meta_effects, access_effects) = split_effects::split_effects(effects);

    if !meta_effects.is_empty() {
        return Err(Status::invalid_argument("Meta effects are not allowed"));
    }

    apply_effects_batch::apply_effects_batch(&transaction_or_db, &access_effects)
        .map_db_err_to_status()?;

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
    // apply_effects_batch(&TransactionOrDb::Db(db), &access_effects)?;
    apply_effects_batch::apply_effects_batch(&transaction_or_db, &access_effects)
        .map_db_err_to_status()?;
    Ok(Response::new(EmptyMessage {}))
}

fn apply_meta_effect(db: &mut TransactionDB, meta_effect: &MetaEffect) -> Result<(), Status> {
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
