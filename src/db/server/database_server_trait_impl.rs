use super::database_server_trait::DatabaseServerTrait;
use super::db_error_to_status::DbErrorOptionToStatus;
use super::db_error_to_status::DbErrorToStatus;
use super::rocks_db_accessor::RocksDbAccessor;
use super::source_sink::effects_sink::EffectsSink;
use crate::db::entity::database_server::DatabaseServer;
use crate::db::entity::reference::database_server_reference::DatabaseServerReference;
use crate::db::entity::reference::database_server_reference::DatabaseServerReferenceTrait;
use crate::ondo_remote::{
    ArrayOfStringResponse, DatabaseServerMessage, DatabaseServerReferenceMessage, EmptyMessage,
    VersionResponse,
};
use tonic::{Request, Response, Status};

impl<'a> Into<DatabaseServerReference> for &'a DatabaseServerReferenceMessage {
    fn into(self) -> DatabaseServerReference {
        DatabaseServerReference
    }
}

impl<'a> Into<DatabaseServer> for &'a DatabaseServerMessage {
    fn into(self) -> DatabaseServer {
        DatabaseServer::default()
    }
}

impl Into<DatabaseServerMessage> for DatabaseServer {
    fn into(self) -> DatabaseServerMessage {
        DatabaseServerMessage {}
    }
}

impl DatabaseServerTrait for RocksDbAccessor {
    fn version(&self, _: Request<EmptyMessage>) -> Result<Response<VersionResponse>, Status> {
        let version = self.get_version();
        let response = VersionResponse {
            major: version.major,
            minor: version.minor,
            patch: version.patch,
            commit: version.commit,
            date: version.date,
            features: version.features,
        };
        Ok(Response::new(response))
    }

    fn create_database_server(
        &self,
        r: Request<DatabaseServerMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        let entity: DatabaseServer = r.get_ref().into();
        entity
            .reference
            .post_database_server(&r.get_ref().into(), self)
            .map_db_err_to_status()?
            .apply_effects(self)
    }

    fn delete_database_server(
        &self,
        r: Request<DatabaseServerReferenceMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        let reference: DatabaseServerReference = r.get_ref().into();
        reference
            .delete_database_server(self, self, self)
            .map_db_err_to_status()?
            .apply_effects(self)
    }

    fn get_database_server(
        &self,
        r: Request<DatabaseServerReferenceMessage>,
    ) -> Result<Response<DatabaseServerMessage>, Status> {
        let reference: DatabaseServerReference = r.get_ref().into();
        reference
            .get_database_server(self)
            .map_db_err_option_to_status()
            .map(|entity| Response::new(entity.into()))
    }

    fn update_database_server(
        &self,
        r: Request<DatabaseServerMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        let entity: DatabaseServer = r.get_ref().into();
        entity
            .reference
            .put_database_server(&entity, self)
            .map_db_err_to_status()?
            .apply_effects(self)
    }

    fn list_domains(
        &self,
        r: Request<DatabaseServerReferenceMessage>,
    ) -> Result<Response<ArrayOfStringResponse>, Status> {
        let reference: DatabaseServerReference = r.get_ref().into();
        let names = reference.list_domain_names(self).map_db_err_to_status()?;
        let response = ArrayOfStringResponse { values: names };
        Ok(Response::new(response))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_server_reference_message_into_database_server_reference() {
        let message = DatabaseServerReferenceMessage {};
        let reference: DatabaseServerReference = (&message).into();
        // Replace with any specific assertions for DatabaseServerReference.
        assert_eq!(reference, DatabaseServerReference);
    }

    #[test]
    fn test_database_server_message_into_database_server() {
        let message = DatabaseServerMessage {};
        let server: DatabaseServer = (&message).into();
        // Replace with any specific assertions for DatabaseServer.
        assert_eq!(server, DatabaseServer::default());
    }

    #[test]
    fn test_database_server_into_database_server_message() {
        let server = DatabaseServer::default();
        let message: DatabaseServerMessage = server.into();
        // Replace with any specific assertions for DatabaseServerMessage.
        assert_eq!(message, DatabaseServerMessage {});
    }
}

