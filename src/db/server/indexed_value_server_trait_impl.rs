use super::indexed_value_server_trait::IndexedValueServerTrait;
use super::rocks_db_accessor::RocksDbAccessor;
use crate::ondo_remote;
use ondo_remote::*;
use tonic::{Request, Response, Status};

impl IndexedValueServerTrait for RocksDbAccessor {
    fn find_values(
        &self,
        _: Request<IndexedValueReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status> {
        todo!()
    }

    fn find_values_by_range(
        &self,
        _: Request<IndexedValueRangeReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status> {
        todo!()
    }
}
