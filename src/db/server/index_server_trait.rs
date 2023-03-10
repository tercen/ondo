use tonic::{Request, Response, Status};
use crate::remote;
use remote::*;

pub trait IndexServerTrait {
    fn create_index(&self, _: Request<IndexMessage>) -> Result<Response<EmptyMessage>, Status>;
    fn delete_index(
        &self,
        _: Request<IndexReferenceMessage>,
    ) -> Result<Response<EmptyMessage>, Status>;
    fn get_index(
        &self,
        _: Request<IndexReferenceMessage>,
    ) -> Result<Response<IndexMessage>, Status>;
    fn update_index(&self, _: Request<IndexMessage>) -> Result<Response<EmptyMessage>, Status>;
}
