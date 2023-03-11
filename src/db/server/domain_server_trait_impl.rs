use super::domain_server_trait::DomainServerTrait;
use super::rocks_db_accessor::RocksDbAccessor;
use crate::ondo_remote::*;
use tonic::{Request, Response, Status};

impl DomainServerTrait for RocksDbAccessor {
    fn create_domain(&self, _: Request<DomainMessage>) -> Result<Response<EmptyMessage>, Status> {
        todo!()
    }

    fn delete_domain(
        &self,
        _: Request<DomainReferenceMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        todo!()
    }

    fn get_domain(
        &self,
        _: Request<DomainReferenceMessage>,
    ) -> Result<Response<DomainMessage>, Status> {
        todo!()
    }

    fn update_domain(&self, _: Request<DomainMessage>) -> Result<Response<EmptyMessage>, Status> {
        todo!()
    }

    fn list_tables(
        &self,
        _: Request<DomainReferenceMessage>,
    ) -> Result<Response<ArrayOfStringResponse>, Status> {
        todo!()
    }
}
