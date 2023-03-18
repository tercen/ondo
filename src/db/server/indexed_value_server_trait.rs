use crate::ondo_remote;
use ondo_remote::*;
use tonic::{Request, Response, Status};

pub trait IndexedValueServerTrait {
    fn find_values(
        &self,
        _: Request<IndexedValueReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status>;
    fn find_values_by_range(
        &self,
        _: Request<IndexedValueRangeReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status>;
}
