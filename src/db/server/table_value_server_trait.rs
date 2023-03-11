use crate::ondo_remote;
use ondo_remote::*;
use tonic::{Request, Response, Status};

pub trait TableValueServerTrait {
    fn create_value(&self, _: Request<TableValueMessage>)
        -> Result<Response<EmptyMessage>, Status>;
    fn delete_value(
        &self,
        _: Request<TableValueReferenceMessage>,
    ) -> Result<Response<EmptyMessage>, Status>;
    fn get_value(
        &self,
        _: Request<TableValueReferenceMessage>,
    ) -> Result<Response<JsonResponse>, Status>;
    fn update_value(&self, _: Request<TableValueMessage>)
        -> Result<Response<EmptyMessage>, Status>;
}
