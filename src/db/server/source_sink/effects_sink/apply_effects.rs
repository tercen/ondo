use crate::db::reference::effect::{Effect, Effects};
use crate::db::server::db_error_to_status::DbErrorToStatus;
use crate::db::server::lockable_db::transaction_maker::TransactionMaker;
use crate::db::DbError;
use crate::ondo_remote::EmptyMessage;
use tonic::{Response, Status};

pub(crate) fn apply_effects(
    ra: &TransactionMaker,
    effects: &Effects,
) -> Result<Response<EmptyMessage>, Status> {
    let cf_opts = rocksdb::Options::default();
    for effect in effects.into_iter() {
        println!("Effect: {:?}", effect);
        match effect {
            Effect::CreateCf(cf_name) => {
                let mut db = ra.write();
                db.create_cf(cf_name, &cf_opts)
                    .map_err(|err| DbError::RocksDbError(err))
                    .map_db_err_to_status()?;
            }
            Effect::DeleteCf(cf_name) => {
                let mut db = ra.write();
                db.drop_cf(cf_name)
                    .map_err(|err| DbError::RocksDbError(err))
                    .map_db_err_to_status()?;
            }
            Effect::DatabaseServerStoredEffect(effect) => {
                super::super::database_server_sink::apply_effect(&ra, effect).map_db_err_to_status()?;
            }
            Effect::DomainStoredEffect(effect) => {
                super::super::domain_sink::apply_effect(&ra, effect).map_db_err_to_status()?;
            }
            Effect::TableStoredEffect(effect) => {
                super::super::table_sink::apply_effect(&ra, effect).map_db_err_to_status()?;
            }
            Effect::TableValueEffect(effect) => {
                super::super::table_value_sink::apply_effect(&ra, effect).map_db_err_to_status()?;
            }
            Effect::IndexValueEffect(effect) => {
                super::super::index_value_sink::apply_effect(&ra, effect).map_db_err_to_status()?;
            }
            Effect::ColumnValueEffect(effect) => {
                super::super::column_value_sink::apply_effect(&ra, effect).map_db_err_to_status()?;
            }
        }
    }
    Ok(Response::new(EmptyMessage {}))
}
