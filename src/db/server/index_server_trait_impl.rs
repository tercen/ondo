use super::index_server_trait::IndexServerTrait;
use super::rocks_db_accessor::RocksDbAccessor;
use crate::ondo_remote::*;
use tonic::{Request, Response, Status};
// FIXME: Implement IndexServerTrait and tests for it.
impl IndexServerTrait for RocksDbAccessor {
    fn create_index(&self, _: Request<IndexMessage>) -> Result<Response<EmptyMessage>, Status> {
        todo!("indexing")
    }

    fn delete_index(
        &self,
        _: Request<IndexReferenceMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        todo!("indexing")
    }

    fn get_index(
        &self,
        _: Request<IndexReferenceMessage>,
    ) -> Result<Response<IndexMessage>, Status> {
        todo!("indexing")
    }

    fn update_index(&self, _: Request<IndexMessage>) -> Result<Response<EmptyMessage>, Status> {
        todo!("indexing")
    }
}
