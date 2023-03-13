use crate::db::server::db_error_to_status::DbErrorToStatus;
use crate::db::{entity::reference::effect::Effect, server::rocks_db_accessor::RocksDbAccessor};
use crate::ondo_remote::EmptyMessage;
use tonic::{Response, Status};

pub(in crate::db::server) trait EffectsSink {
    fn apply_effects(&self, ra: &RocksDbAccessor) -> Result<Response<EmptyMessage>, Status>;
}

impl EffectsSink for Vec<Effect> {
    fn apply_effects(&self, ra: &RocksDbAccessor) -> Result<Response<EmptyMessage>, Status> {
        let _db = ra.db_guard().map_db_err_to_status()?;
        self.into_iter().for_each(|effect| match effect {
            Effect::CreateCf(_cf_name) => {
                todo!();
                // db.create_cf(cf_name).map_db_err_to_status();
            }
            Effect::DeleteCf(_) => {
                todo!();
            }
            Effect::DatabaseServerStoredEffect(_) => {
                todo!();
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
        });
        Ok(Response::new(EmptyMessage {}))
    }
}
