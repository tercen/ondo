mod apply_effects;
mod queue_tasks;

use crate::db::reference::effect::Effects;
use crate::db::server::lockable_db::transaction_maker::LockableTransactionOrDb;
use crate::db::tasks::task::Tasks;
use crate::ondo_remote::EmptyMessage;
use tonic::{Response, Status};

pub(in crate::db::server) trait EffectsTasksSink {
    fn apply_effects_queue_tasks(
        &self,
        ra: &LockableTransactionOrDb,
    ) -> Result<Response<EmptyMessage>, Status>;
    fn apply_effects_apply_tasks(
        &self,
        ra: &LockableTransactionOrDb,
    ) -> Result<Response<EmptyMessage>, Status>;
}

impl EffectsTasksSink for (Effects, Tasks) {
    fn apply_effects_queue_tasks(
        &self,
        ra: &LockableTransactionOrDb,
    ) -> Result<Response<EmptyMessage>, Status> {
        let (effects, tasks) = self;
        apply_effects::apply_effects(ra, effects)?;
        queue_tasks::queue_tasks(ra, tasks)?;
        Ok(Response::new(EmptyMessage {}))
    }
    fn apply_effects_apply_tasks(
        &self,
        ra: &LockableTransactionOrDb,
    ) -> Result<Response<EmptyMessage>, Status> {
        let (effects, tasks) = self;
        apply_effects::apply_effects(ra, effects)?;
        queue_tasks::apply_tasks(ra, tasks)?;
        Ok(Response::new(EmptyMessage {}))
    }
}

pub(in crate::db::server) trait EffectsSink {
    fn apply_effects(&self, ra: &LockableTransactionOrDb) -> Result<Response<EmptyMessage>, Status>;
}

impl EffectsSink for Effects {
    fn apply_effects(&self, ra: &LockableTransactionOrDb) -> Result<Response<EmptyMessage>, Status> {
        apply_effects::apply_effects(ra, self)
    }
}
