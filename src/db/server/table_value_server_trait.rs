use crate::ondo_remote;
use ondo_remote::*;
use tonic::{Request, Response, Status};

pub trait TableValueServerTrait {
    fn create_value(
        &self,
        r: Request<CreateTableValueMessage>,
    ) -> Result<Response<OndoKeyMessage>, Status>;
    fn delete_value(
        &self,
        r: Request<TableValueReferenceMessage>,
    ) -> Result<Response<EmptyMessage>, Status>;
    fn get_value(
        &self,
        r: Request<TableValueReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status>;
    fn update_value(&self, r: Request<TableValueMessage>)
        -> Result<Response<EmptyMessage>, Status>;
}
