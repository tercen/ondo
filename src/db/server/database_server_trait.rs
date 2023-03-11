use crate::ondo_remote;
use ondo_remote::*;
use tonic::{Request, Response, Status};

pub trait DatabaseServerTrait {
    fn version(&self, _: Request<EmptyMessage>) -> Result<Response<VersionResponse>, Status>;
    fn create_database_server(
        &self,
        _: Request<DatabaseServerMessage>,
    ) -> Result<Response<EmptyMessage>, Status>;
    fn delete_database_server(
        &self,
        _: Request<DatabaseServerReferenceMessage>,
    ) -> Result<Response<EmptyMessage>, Status>;
    fn get_database_server(
        &self,
        _: Request<DatabaseServerReferenceMessage>,
    ) -> Result<Response<DatabaseServerMessage>, Status>;
    fn update_database_server(
        &self,
        _: Request<DatabaseServerMessage>,
    ) -> Result<Response<EmptyMessage>, Status>;
    fn list_domains(
        &self,
        _: Request<EmptyMessage>,
    ) -> Result<Response<ArrayOfStringResponse>, Status>;
}
