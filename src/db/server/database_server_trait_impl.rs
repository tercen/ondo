use super::database_server_trait::DatabaseServerTrait;
use super::db_error_to_status::DbErrorOptionToStatus;
use super::db_error_to_status::DbErrorToStatus;
use super::lockable_db::transaction_or_db::TransactionOrDb;
use super::lockable_db::version::Version;
use super::source_sink::effects_sink::EffectsSink;
use crate::db::{
    entity::DatabaseServer,
    reference::database_server_reference::{DatabaseServerReference, DatabaseServerReferenceTrait},
};
use crate::ondo_remote::{
    ArrayOfStringResponse, DatabaseServerMessage, DatabaseServerReferenceMessage, EmptyMessage,
    VersionResponse,
};
use rocksdb::TransactionDB;
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

impl DatabaseServerTrait for TransactionDB {
    fn version(&self, _: Request<EmptyMessage>) -> Result<Response<VersionResponse>, Status> {
        let version = Version::new();
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
        &mut self,
        r: Request<DatabaseServerMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        let entity: DatabaseServer = r.get_ref().into();
        let transaction_or_db = TransactionOrDb::Db(self);
        entity
            .reference
            .post_database_server(&r.get_ref().into(), &transaction_or_db)
            .map_db_err_to_status()?
            .apply_all_effects(self)
    }

    fn delete_database_server(
        &mut self,
        r: Request<DatabaseServerReferenceMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        let reference: DatabaseServerReference = r.get_ref().into();
        let transaction_or_db = TransactionOrDb::Db(self);
        reference
            .delete_database_server(&transaction_or_db, &transaction_or_db, &transaction_or_db)
            .map_db_err_to_status()?
            .apply_all_effects(self)
    }

    fn get_database_server(
        &self,
        r: Request<DatabaseServerReferenceMessage>,
    ) -> Result<Response<DatabaseServerMessage>, Status> {
        let reference: DatabaseServerReference = r.get_ref().into();
        let transaction_or_db = TransactionOrDb::Db(self);
        reference
            .get_database_server(&transaction_or_db)
            .map_db_err_option_to_status()
            .map(|entity| Response::new(entity.into()))
    }

    fn update_database_server(
        &mut self,
        r: Request<DatabaseServerMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        let entity: DatabaseServer = r.get_ref().into();
        let transaction_or_db = TransactionOrDb::Db(self);
        entity
            .reference
            .put_database_server(&entity, &transaction_or_db)
            .map_db_err_to_status()?
            .apply_all_effects(self)
    }

    fn list_domains(
        &self,
        r: Request<DatabaseServerReferenceMessage>,
    ) -> Result<Response<ArrayOfStringResponse>, Status> {
        let reference: DatabaseServerReference = r.get_ref().into();
        let transaction_or_db = TransactionOrDb::Db(self);
        let names = reference
            .list_domain_names(&transaction_or_db)
            .map_db_err_to_status()?;
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
