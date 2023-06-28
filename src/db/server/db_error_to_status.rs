use crate::db::db_error::*;
use tonic::Status;

pub(crate) fn db_error_to_status(err: DbError) -> Status {
    let db_error_message = err.to_string();
    let db_error_code = u32::from(err);
    let status_message = format!("Database error {}: {}", db_error_code, db_error_message);
    Status::unknown(status_message)
}

fn map_db_error_to_status<T>(r: DbResult<T>) -> Result<T, Status> {
    match r {
        Ok(t) => Ok(t),
        Err(e) => Err(db_error_to_status(e)),
    }
}

fn map_db_none_to_status<T>(opt: Option<T>) -> Result<T, Status> {
    match opt {
        Some(t) => Ok(t),
        None => Err(Status::not_found("Not found")),
    }
}

pub(crate) trait DbErrorToStatus<T> {
    fn map_db_err_to_status(self) -> Result<T, Status>;
}

impl<T> DbErrorToStatus<T> for DbResult<T> {
    fn map_db_err_to_status(self) -> Result<T, Status> {
        map_db_error_to_status(self)
    }
}

pub(crate) trait DbErrorOptionToStatus<T> {
    fn map_db_err_option_to_status(self) -> Result<T, Status>;
}

impl<T> DbErrorOptionToStatus<T> for DbResult<Option<T>> {
    fn map_db_err_option_to_status(self) -> Result<T, Status> {
        map_db_none_to_status(map_db_error_to_status(self)?)
    }
}
