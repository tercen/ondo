use super::send_response::send_response;
use crate::db::server::database_server_trait::DatabaseServerTrait;
use crate::db::server::lockable_db::transaction_maker::LockableTransactionOrDb;
use crate::ondo_remote::{
    database_server_ops::RequestType, transaction_response::ResponseType, DatabaseServerMessage,
    DatabaseServerReferenceMessage, TransactionResponse,
};
use tonic::Status;

pub(crate) struct DatabaseServerOpsSubServer<'a> {
    pub lockable_db: LockableTransactionOrDb<'a>,
}

impl<'a> DatabaseServerOpsSubServer<'a> {
    pub fn process_request(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        request: RequestType,
    ) {
        match request {
            RequestType::CreateRequest(create_request) => {
                self.create_database(tx, create_request);
            }
            RequestType::DeleteRequest(delete_request) => {
                self.delete_database(tx, delete_request);
            }
            RequestType::GetRequest(get_request) => {
                self.get_database(tx, get_request);
            }
            RequestType::UpdateRequest(update_request) => {
                self.update_database(tx, update_request);
            }
            RequestType::ListDomainsRequest(list_domains_request) => {
                self.list_domains(tx, list_domains_request);
            }
        }
    }

    async fn create_database(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        create_request: DatabaseServerMessage,
    ) {
        let result = self
            .lockable_db
            .create_database_server(tonic::Request::new(create_request));
        let response_type = match result {
            Ok(response) => ResponseType::EmptyResponse(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        send_response(tx, response_type)
    }

    async fn delete_database(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        delete_request: DatabaseServerReferenceMessage,
    ) {
        let result = self
            .lockable_db
            .delete_database_server(tonic::Request::new(delete_request));
        let response_type = match result {
            Ok(response) => ResponseType::EmptyResponse(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        send_response(tx, response_type)
    }

    async fn get_database(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        get_request: DatabaseServerReferenceMessage,
    ) {
        let result = self
            .lockable_db
            .get_database_server(tonic::Request::new(get_request));
        let response_type = match result {
            Ok(response) => ResponseType::DatabaseServerMessage(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        send_response(tx, response_type)
    }

    async fn update_database(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        update_request: DatabaseServerMessage,
    ) {
        let result = self
            .lockable_db
            .update_database_server(tonic::Request::new(update_request));
        let response_type = match result {
            Ok(response) => ResponseType::EmptyResponse(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        send_response(tx, response_type)
    }

    async fn list_domains(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        list_domains_request: DatabaseServerReferenceMessage,
    ) {
        let result = self
            .lockable_db
            .list_domains(tonic::Request::new(list_domains_request));
        let response_type = match result {
            Ok(response) => ResponseType::ArrayOfStringResponse(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        send_response(tx, response_type)
    }
}
