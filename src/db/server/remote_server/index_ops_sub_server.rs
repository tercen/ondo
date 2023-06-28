use crate::db::server::index_server_trait::IndexServerTrait;

use crate::ondo_remote::{
    index_ops::RequestType, transaction_response::ResponseType, IndexMessage,
    IndexReferenceMessage, TransactionResponse,
};
use rocksdb::TransactionDB;
use tonic::Status;

pub(crate) struct IndexOpsSubServer<'a> {
    pub db: &'a mut TransactionDB,
}

impl<'a> IndexOpsSubServer<'a> {
    pub fn process_request(
        &mut self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        request: RequestType,
    ) -> ResponseType {
        match request {
            RequestType::CreateRequest(create_request) => self.create_index(tx, create_request),
            RequestType::DeleteRequest(delete_request) => self.delete_index(tx, delete_request),
            RequestType::GetRequest(get_request) => self.get_index(tx, get_request),
            RequestType::UpdateRequest(update_request) => self.update_index(tx, update_request),
        }
    }

    fn create_index(
        &mut self,
        _tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        create_request: IndexMessage,
    ) -> ResponseType {
        let result = self.db.create_index(tonic::Request::new(create_request));
        let response_type = match result {
            Ok(response) => ResponseType::EmptyResponse(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        response_type
    }

    fn delete_index(
        &mut self,
        _tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        delete_request: IndexReferenceMessage,
    ) -> ResponseType {
        let result = self.db.delete_index(tonic::Request::new(delete_request));
        let response_type = match result {
            Ok(response) => ResponseType::EmptyResponse(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        response_type
    }

    fn get_index(
        &self,
        _tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        get_request: IndexReferenceMessage,
    ) -> ResponseType {
        let result = self.db.get_index(tonic::Request::new(get_request));
        let response_type = match result {
            Ok(response) => ResponseType::IndexMessage(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        response_type
    }

    fn update_index(
        &mut self,
        _tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        update_request: IndexMessage,
    ) -> ResponseType {
        let result = self.db.update_index(tonic::Request::new(update_request));
        let response_type = match result {
            Ok(response) => ResponseType::EmptyResponse(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        response_type
    }
}
