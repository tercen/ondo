mod apply_effects;
mod queue_tasks;

use crate::db::{
    reference::effect::Effects, server::lockable_db::transaction_or_db::TransactionOrDb,
};

use crate::db::tasks::task::Tasks;
use crate::ondo_remote::EmptyMessage;
use rocksdb::TransactionDB;
use tonic::{Response, Status};

pub(in crate::db::server) trait EffectsTasksSink {
    fn apply_effects_queue_tasks<'a>(
        self,
        ra: &TransactionOrDb<'a>,
    ) -> Result<Response<EmptyMessage>, Status>;

    fn apply_effects_apply_tasks<'a>(
        self,
        ra: &TransactionOrDb<'a>,
    ) -> Result<Response<EmptyMessage>, Status>;
}

impl EffectsTasksSink for (Effects, Tasks) {
    fn apply_effects_queue_tasks<'a>(
        self,
        ra: &TransactionOrDb<'a>,
    ) -> Result<Response<EmptyMessage>, Status> {
        let (effects, tasks) = self;
        apply_effects::apply_effects(ra, effects)?;
        queue_tasks::queue_tasks(ra, &tasks)?;
        Ok(Response::new(EmptyMessage {}))
    }

    fn apply_effects_apply_tasks<'a>(
        self,
        ra: &TransactionOrDb<'a>,
    ) -> Result<Response<EmptyMessage>, Status> {
        let (effects, tasks) = self;
        apply_effects::apply_effects(ra, effects)?;
        queue_tasks::apply_tasks(ra, &tasks)?;
        Ok(Response::new(EmptyMessage {}))
    }
}

pub(in crate::db::server) trait EffectsSink {
    fn apply_effects<'a>(self, ra: &TransactionOrDb<'a>)
        -> Result<Response<EmptyMessage>, Status>;
    fn apply_all_effects<'a>(
        self,
        ra: &mut TransactionDB,
    ) -> Result<Response<EmptyMessage>, Status>;
}

impl EffectsSink for Effects {
    fn apply_effects<'a>(
        self,
        ra: &TransactionOrDb<'a>,
    ) -> Result<Response<EmptyMessage>, Status> {
        apply_effects::apply_effects(ra, self)
    }
    fn apply_all_effects<'a>(
        self,
        ra: &mut TransactionDB,
    ) -> Result<Response<EmptyMessage>, Status> {
        apply_effects::apply_all_effects(ra, self)
    }
}
