use super::indexed_value_server_trait::IndexedValueServerTrait;
use super::rocks_db_accessor::RocksDbAccessor;
use crate::remote;
use remote::*;
use tonic::{Request, Response, Status};

impl IndexedValueServerTrait for RocksDbAccessor {
    fn find_values(
        &self,
        _: Request<IndexedValueReferenceMessage>,
    ) -> Result<Response<JsonResponse>, Status> {
        todo!()
    }

    fn find_values_by_range(
        &self,
        _: Request<IndexedValueRangeReferenceMessage>,
    ) -> Result<Response<JsonResponse>, Status> {
        todo!()
    }
}
