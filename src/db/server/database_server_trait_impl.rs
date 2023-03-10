use super::database_server_trait::DatabaseServerTrait;
use super::rocks_db_accessor::RocksDbAccessor;
use crate::remote::{
    ArrayOfStringResponse, DatabaseServerMessage, DatabaseServerReferenceMessage, EmptyMessage,
    VersionResponse,
};
use tonic::{Request, Response, Status};

impl DatabaseServerTrait for RocksDbAccessor {
    fn version(&self, _: Request<EmptyMessage>) -> Result<Response<VersionResponse>, Status> {
        let response = VersionResponse { version: "0".to_owned() };
        Ok(Response::new(response))
    }

    fn create_database_server(
        &self,
        _: Request<DatabaseServerMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        todo!()
    }

    fn delete_database_server(
        &self,
        _: Request<DatabaseServerReferenceMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        todo!()
    }

    fn get_database_server(
        &self,
        _: Request<DatabaseServerReferenceMessage>,
    ) -> Result<Response<DatabaseServerMessage>, Status> {
        todo!()
    }

    fn update_database_server(
        &self,
        _: Request<DatabaseServerMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        todo!()
    }

    fn list_domains(
        &self,
        _: Request<EmptyMessage>,
    ) -> Result<Response<ArrayOfStringResponse>, Status> {
        todo!()
    }
}
