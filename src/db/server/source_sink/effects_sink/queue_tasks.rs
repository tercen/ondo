use log::warn;
use crate::db::tasks::task::Tasks;
// use crate::db::server::db_error_to_status::DbErrorToStatus;
use crate::db::server::lockable_db::transaction_maker::TransactionMaker;
// use crate::db::DbError;
use crate::ondo_remote::EmptyMessage;
use tonic::{Response, Status};

pub(crate) fn queue_tasks(
    ra: &TransactionMaker,
    tasks: &Tasks,
) -> Result<Response<EmptyMessage>, Status> {
    warn!("Tasks are executed instead of queuing");
    apply_tasks(ra, tasks) //FIXME: queue_tasks should queue tasks
}

pub(crate) fn apply_tasks(
    //For testing
    _ra: &TransactionMaker,
    _tasks: &Tasks,
) -> Result<Response<EmptyMessage>, Status> {
    warn!("apply_tasks not implemented"); //FIXME implement apply_tasks
    Ok(Response::new(EmptyMessage {}))
}
