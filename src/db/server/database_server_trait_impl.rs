use super::database_server_trait::DatabaseServerTrait;
use super::db_error_to_status::DbErrorOptionToStatus;
use super::db_error_to_status::DbErrorToStatus;
use super::rocks_db_accessor::RocksDbAccessor;
use super::source_sink::effects_sink::EffectsSink;
use super::to_entity_trait::FromEntity;
use super::to_entity_trait::ToEntity;
use super::to_reference_trait::ToReference;
use crate::db::entity::database_server::DatabaseServer;
use crate::db::entity::reference::database_server_reference::DatabaseServerReference;
use crate::db::entity::reference::database_server_reference::DatabaseServerReferenceTrait;
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

impl FromEntity<DatabaseServer> for Response<DatabaseServerMessage> {
    fn from_entity(_entity: DatabaseServer) -> Self {
        Response::new(DatabaseServerMessage {})
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
        r.to_reference()
            .post_database_server(&r.to_entity(), self)
            .map_db_err_to_status()?
            .apply_effects(self)
    }

    fn delete_database_server(
        &self,
        r: Request<DatabaseServerReferenceMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        r.to_reference()
            .delete_database_server(self, self, self)
            .map_db_err_to_status()?
            .apply_effects(self)
    }

    fn get_database_server(
        &self,
        r: Request<DatabaseServerReferenceMessage>,
    ) -> Result<Response<DatabaseServerMessage>, Status> {
        r.to_reference()
            .get_database_server(self)
            .map_db_err_option_to_status()
            .map(|entity| Response::<DatabaseServerMessage>::from_entity(entity))
    }

    fn update_database_server(
        &self,
        r: Request<DatabaseServerMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        r.to_reference()
            .put_database_server(&r.to_entity(), self)
            .map_db_err_to_status()?
            .apply_effects(self)
    }

    fn list_domains(
        &self,
        r: Request<DatabaseServerReferenceMessage>,
    ) -> Result<Response<ArrayOfStringResponse>, Status> {
        let names = r
            .to_reference()
            .list_domain_names(self)
            .map_db_err_to_status()?;
        let response = ArrayOfStringResponse { values: names };
        Ok(Response::new(response))
    }
}
