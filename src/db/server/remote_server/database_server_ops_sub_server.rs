// use super::send_response::send_response;
use crate::db::server::database_server_trait::DatabaseServerTrait;

use crate::ondo_remote::{
    database_server_ops::RequestType, transaction_response::ResponseType, DatabaseServerMessage,
    DatabaseServerReferenceMessage, TransactionResponse,
};
use rocksdb::TransactionDB;
use tonic::Status;

pub(crate) struct DatabaseServerOpsSubServer<'a> {
    pub db: &'a mut TransactionDB,
}

impl<'a> DatabaseServerOpsSubServer<'a> {
    pub fn process_request(
        &mut self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        request: RequestType,
    ) -> ResponseType {
        match request {
            RequestType::CreateRequest(create_request) => self.create_database(tx, create_request),
            RequestType::DeleteRequest(delete_request) => self.delete_database(tx, delete_request),
            RequestType::GetRequest(get_request) => self.get_database(tx, get_request),
            RequestType::UpdateRequest(update_request) => self.update_database(tx, update_request),
            RequestType::ListDomainsRequest(list_domains_request) => {
                self.list_domains(tx, list_domains_request)
            }
        }
    }

    fn create_database(
        &mut self,
        _tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        create_request: DatabaseServerMessage,
    ) -> ResponseType {
        let result = self
            .db
            .create_database_server(tonic::Request::new(create_request));
        let response_type = match result {
            Ok(response) => ResponseType::EmptyResponse(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        response_type
    }

    fn delete_database(
        &mut self,
        _tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        delete_request: DatabaseServerReferenceMessage,
    ) -> ResponseType {
        let result = self
            .db
            .delete_database_server(tonic::Request::new(delete_request));
        let response_type = match result {
            Ok(response) => ResponseType::EmptyResponse(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        response_type
    }

    fn get_database(
        &self,
        _tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        get_request: DatabaseServerReferenceMessage,
    ) -> ResponseType {
        let result = self
            .db
            .get_database_server(tonic::Request::new(get_request));
        let response_type = match result {
            Ok(response) => ResponseType::DatabaseServerMessage(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        response_type
    }

    fn update_database(
        &mut self,
        _tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        update_request: DatabaseServerMessage,
    ) -> ResponseType {
        let result = self
            .db
            .update_database_server(tonic::Request::new(update_request));
        let response_type = match result {
            Ok(response) => ResponseType::EmptyResponse(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        response_type
    }

    fn list_domains(
        &self,
        _tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        list_domains_request: DatabaseServerReferenceMessage,
    ) -> ResponseType {
        let result = self
            .db
            .list_domains(tonic::Request::new(list_domains_request));
        let response_type = match result {
            Ok(response) => ResponseType::ArrayOfStringResponse(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        response_type
    }
}
