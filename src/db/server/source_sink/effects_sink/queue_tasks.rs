use crate::db::{tasks::task::Tasks, server::lockable_db::transaction_or_db::TransactionOrDb};
use log::warn;
// use crate::db::server::db_error_to_status::DbErrorToStatus;

// use crate::db::DbError;
use crate::ondo_remote::EmptyMessage;
use tonic::{Response, Status};

pub(crate) fn queue_tasks<'a>(
    ra: &TransactionOrDb<'a>,
    tasks: &Tasks,
) -> Result<Response<EmptyMessage>, Status> {
    warn!("Tasks are executed instead of queuing"); //FIXME use task queue
    apply_tasks(ra, tasks) //FIXME: queue_tasks should queue tasks
}

pub(crate) fn apply_tasks<'a>(
    //For testing
    _ra: &TransactionOrDb<'a>,
    _tasks: &Tasks,
) -> Result<Response<EmptyMessage>, Status> {
    warn!("apply_tasks not implemented"); //FIXME implement apply_tasks
    Ok(Response::new(EmptyMessage {}))
}
