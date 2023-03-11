use crate::ondo_remote;
use ondo_remote::*;
use tonic::{Request, Response, Status};

pub trait DomainServerTrait {
    fn create_domain(&self, _: Request<DomainMessage>) -> Result<Response<EmptyMessage>, Status>;
    fn delete_domain(
        &self,
        _: Request<DomainReferenceMessage>,
    ) -> Result<Response<EmptyMessage>, Status>;
    fn get_domain(
        &self,
        _: Request<DomainReferenceMessage>,
    ) -> Result<Response<DomainMessage>, Status>;
    fn update_domain(&self, _: Request<DomainMessage>) -> Result<Response<EmptyMessage>, Status>;
    fn list_tables(
        &self,
        _: Request<DomainReferenceMessage>,
    ) -> Result<Response<ArrayOfStringResponse>, Status>;
}
