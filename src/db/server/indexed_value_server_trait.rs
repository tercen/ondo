use tonic::{Request, Response, Status};
use crate::remote;
use remote::*;

pub trait IndexedValueServerTrait {
    fn find_values(
        &self,
        _: Request<IndexedValueReferenceMessage>,
    ) -> Result<Response<JsonResponse>, Status>;
    fn find_values_by_range(
        &self,
        _: Request<IndexedValueRangeReferenceMessage>,
    ) -> Result<Response<JsonResponse>, Status>;
}
