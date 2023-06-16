use super::send_response::send_response;
use crate::db::server::lockable_db::transaction_maker::LockableTransactionOrDb;
use crate::db::server::text_index_server_trait::TextIndexServerTrait;
use crate::ondo_remote::{
    text_index_ops::RequestType, transaction_response::ResponseType, TantivyQueryMessage,
    TextIndexMessage, TextIndexReferenceMessage, TransactionResponse,
};
use tonic::Status;

pub(crate) struct TextIndexOpsSubServer<'a> {
    pub lockable_db: LockableTransactionOrDb<'a>,
}

impl<'a> TextIndexOpsSubServer<'a> {
    pub fn process_request(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        request: RequestType,
    ) {
        match request {
            RequestType::CreateRequest(create_request) => {
                self.create_text_index(tx, create_request);
            }
            RequestType::DeleteRequest(delete_request) => {
                self.delete_text_index(tx, delete_request);
            }
            RequestType::GetRequest(get_request) => {
                self.get_text_index(tx, get_request);
            }
            RequestType::UpdateRequest(update_request) => {
                self.update_text_index(tx, update_request);
            }
            RequestType::SearchRequest(search_request) => {
                self.search_text_index(tx, search_request);
            }
        }
    }

    fn create_text_index(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        create_request: TextIndexMessage,
    ) {
        let result = self
            .lockable_db
            .create_text_index(tonic::Request::new(create_request));
        let response_type = match result {
            Ok(response) => ResponseType::EmptyResponse(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
    }

    async fn delete_text_index(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        delete_request: TextIndexReferenceMessage,
    ) {
        let result = self
            .lockable_db
            .delete_text_index(tonic::Request::new(delete_request));
        let response_type = match result {
            Ok(response) => ResponseType::EmptyResponse(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
    }

    fn get_text_index(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        get_request: TextIndexReferenceMessage,
    ) {
        let result = self
            .lockable_db
            .get_text_index(tonic::Request::new(get_request));
        let response_type = match result {
            Ok(response) => ResponseType::TextIndexMessage(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        send_response(tx, response_type);
    }

    fn update_text_index(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        update_request: TextIndexMessage,
    ) {
        let result = self
            .lockable_db
            .update_text_index(tonic::Request::new(update_request));
        let response_type = match result {
            Ok(response) => ResponseType::EmptyResponse(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
    }

    fn search_text_index(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        search_request: TantivyQueryMessage,
    ) {
        let result = self
            .lockable_db
            .search_text_index(tonic::Request::new(search_request));
        let response_type = match result {
            Ok(response) => ResponseType::JsonMessage(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        send_response(tx, response_type);
    }
}
