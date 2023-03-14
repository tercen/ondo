use crate::db::db_error::DbError;
use crate::db::server::db_error_to_status::DbErrorToStatus;
use crate::db::{entity::reference::effect::Effect, server::rocks_db_accessor::RocksDbAccessor};
use crate::ondo_remote::EmptyMessage;
use tonic::{Response, Status};

pub(in crate::db::server) trait EffectsSink {
    fn apply_effects(&self, ra: &RocksDbAccessor) -> Result<Response<EmptyMessage>, Status>;
}

impl EffectsSink for Vec<Effect> {
    fn apply_effects(&self, ra: &RocksDbAccessor) -> Result<Response<EmptyMessage>, Status> {
        let cf_opts = rocksdb::Options::default();
        for effect in self.into_iter() {
            println!("Effect: {:?}", effect);
            match effect {
                Effect::CreateCf(cf_name) => {
                    let guarded_db = ra.guarded_db();
                    let mut db = guarded_db
                        .write()
                        .map_err(|_| DbError::CanNotLockDbMutex)
                        .map_db_err_to_status()?;
                    db.create_cf(cf_name, &cf_opts)
                        .map_err(|err| DbError::RocksDbError(err))
                        .map_db_err_to_status()?;
                }
                Effect::DeleteCf(cf_name) => {
                    let guarded_db = ra.guarded_db();
                    let mut db = guarded_db
                        .write()
                        .map_err(|_| DbError::CanNotLockDbMutex)
                        .map_db_err_to_status()?;
                    db.drop_cf(cf_name)
                        .map_err(|err| DbError::RocksDbError(err))
                        .map_db_err_to_status()?;
                }
                Effect::DatabaseServerStoredEffect(effect) => {
                    super::database_server_sink::apply_effect(&ra, effect)
                        .map_db_err_to_status()?;
                }
                Effect::DomainStoredEffect(_) => {
                    todo!();
                }
                Effect::TableStoredEffect(_) => {
                    todo!();
                }
                Effect::TableValueEffect(_) => {
                    todo!();
                }
                Effect::IndexValueEffect(_) => {
                    todo!();
                }
                Effect::ColumnValueEffect(_) => {
                    todo!();
                }
            }
        }
        Ok(Response::new(EmptyMessage {}))
    }
}