use super::database_server_trait::DatabaseServerTrait;
use super::db_error_to_status::{DbErrorOptionToStatus, DbErrorToStatus};
use super::effects_sink::EffectSink;
use super::rocks_db_accessor::RocksDbAccessor;
use super::to_entity_trait::ToEntity;
use super::to_reference_trait::ToReference;
use crate::db::entity::database_server::DatabaseServer;
use crate::db::entity::reference::database_server_reference::{
    DatabaseServerReference, DatabaseServerReferenceTrait,
};
use crate::ondo_remote::{
    ArrayOfStringResponse, DatabaseServerMessage, DatabaseServerReferenceMessage, EmptyMessage,
    VersionResponse,
};
use tonic::{Request, Response, Status};

impl ToReference<DatabaseServerReference> for Request<DatabaseServerReferenceMessage> {
    fn to_reference(&self) -> DatabaseServerReference {
        DatabaseServerReference
    }
}

impl ToReference<DatabaseServerReference> for Request<DatabaseServerMessage> {
    fn to_reference(&self) -> DatabaseServerReference {
        DatabaseServerReference
    }
}

impl ToEntity<DatabaseServer> for Request<DatabaseServerMessage> {
    fn to_entity(&self) -> DatabaseServer {
        DatabaseServer
    }
}

impl DatabaseServerTrait for RocksDbAccessor {
    fn version(&self, _: Request<EmptyMessage>) -> Result<Response<VersionResponse>, Status> {
        let response = VersionResponse {
            version: "0".to_owned(),
        };
        Ok(Response::new(response))
    }

    fn create_database_server(
        &self,
        r: Request<DatabaseServerMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        todo!();
        // r.to_reference()
        //     .post_database_server(&r.to_entity(), requests)
        //     .map_db_err_to_status()?
        //     .apply_effects()
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
