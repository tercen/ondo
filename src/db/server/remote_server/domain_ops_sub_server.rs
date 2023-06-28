use crate::db::server::domain_server_trait::DomainServerTrait;

use crate::ondo_remote::{
    domain_ops::RequestType, transaction_response::ResponseType, DomainMessage,
    DomainReferenceMessage, TransactionResponse,
};
use rocksdb::TransactionDB;
use tonic::Status;

pub(crate) struct DomainOpsSubServer<'a> {
    pub db: &'a mut TransactionDB,
}

impl<'a> DomainOpsSubServer<'a> {
    pub fn process_request(
        &mut self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        request: RequestType,
    ) -> ResponseType {
        match request {
            RequestType::CreateRequest(create_request) => self.create_domain(tx, create_request),
            RequestType::DeleteRequest(delete_request) => self.delete_domain(tx, delete_request),
            RequestType::GetRequest(get_request) => self.get_domain(tx, get_request),
            RequestType::UpdateRequest(update_request) => self.update_domain(tx, update_request),
            RequestType::ListTablesRequest(list_tables_request) => {
                self.list_tables(tx, list_tables_request)
            }
        }
    }

    fn create_domain(
        &mut self,
        _tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        create_request: DomainMessage,
    ) -> ResponseType {
        let result = self.db.create_domain(tonic::Request::new(create_request));
        let response_type = match result {
            Ok(response) => ResponseType::EmptyResponse(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        response_type
    }

    fn delete_domain(
        &mut self,
        _tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        delete_request: DomainReferenceMessage,
    ) -> ResponseType {
        let result = self.db.delete_domain(tonic::Request::new(delete_request));
        let response_type = match result {
            Ok(response) => ResponseType::EmptyResponse(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        response_type
    }

    fn get_domain(
        &self,
        _tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        get_request: DomainReferenceMessage,
    ) -> ResponseType {
        let result = self.db.get_domain(tonic::Request::new(get_request));
        let response_type = match result {
            Ok(response) => ResponseType::DomainMessage(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        response_type
    }

    fn update_domain(
        &mut self,
        _tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        update_request: DomainMessage,
    ) -> ResponseType {
        let result = self.db.update_domain(tonic::Request::new(update_request));
        let response_type = match result {
            Ok(response) => ResponseType::EmptyResponse(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        response_type
    }

    fn list_tables(
        &self,
        _tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        list_tables_request: DomainReferenceMessage,
    ) -> ResponseType {
        let result = self
            .db
            .list_tables(tonic::Request::new(list_tables_request));
        let response_type = match result {
            Ok(response) => ResponseType::ArrayOfStringResponse(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        response_type
    }
}
