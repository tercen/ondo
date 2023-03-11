use super::index_server_trait::IndexServerTrait;
use super::rocks_db_accessor::RocksDbAccessor;
use crate::ondo_remote::*;
use tonic::{Request, Response, Status};

impl IndexServerTrait for RocksDbAccessor {
    fn create_index(&self, _: Request<IndexMessage>) -> Result<Response<EmptyMessage>, Status> {
        todo!()
    }

    fn delete_index(
        &self,
        _: Request<IndexReferenceMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        todo!()
    }

    fn get_index(
        &self,
        _: Request<IndexReferenceMessage>,
    ) -> Result<Response<IndexMessage>, Status> {
        todo!()
    }

    fn update_index(&self, _: Request<IndexMessage>) -> Result<Response<EmptyMessage>, Status> {
        todo!()
    }
}
