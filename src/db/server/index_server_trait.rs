use crate::ondo_remote;
use ondo_remote::*;
use tonic::{Request, Response, Status};

pub trait IndexServerTrait {
    fn create_index(&mut self, _: Request<IndexMessage>) -> Result<Response<EmptyMessage>, Status>;
    fn delete_index(
        &mut self,
        r: Request<IndexReferenceMessage>,
    ) -> Result<Response<EmptyMessage>, Status>;
    fn get_index(
        &self,
        r: Request<IndexReferenceMessage>,
    ) -> Result<Response<IndexMessage>, Status>;
    fn update_index(&mut self, _: Request<IndexMessage>) -> Result<Response<EmptyMessage>, Status>;
}

pub trait IndexedValueServerTrait {
    fn find_values(
        &self,
        r: Request<IndexedValueReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status>;

    fn find_values_by_range(
        &self,
        r: Request<IndexedValueRangeReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status>;
}
