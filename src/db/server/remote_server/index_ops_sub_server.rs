use super::send_response::send_response;
use crate::db::server::index_server_trait::IndexServerTrait;
use crate::db::server::lockable_db::transaction_maker::LockableTransactionOrDb;
use crate::ondo_remote::{
    index_ops::RequestType, transaction_response::ResponseType, IndexMessage,
    IndexReferenceMessage, TransactionResponse,
};
use tonic::Status;

pub(crate) struct IndexOpsSubServer<'a> {
    pub lockable_db: LockableTransactionOrDb<'a>,
}

impl<'a> IndexOpsSubServer<'a> {
    pub async fn process_request(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        request: RequestType,
    ) {
        match request {
            RequestType::CreateRequest(create_request) => {
                self.create_index(tx, create_request).await;
            }
            RequestType::DeleteRequest(delete_request) => {
                self.delete_index(tx, delete_request).await;
            }
            RequestType::GetRequest(get_request) => {
                self.get_index(tx, get_request).await;
            }
            RequestType::UpdateRequest(update_request) => {
                self.update_index(tx, update_request).await;
            }
        }
    }

    async fn create_index(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        create_request: IndexMessage,
    ) {
        let result = self
            .lockable_db
            .create_index(tonic::Request::new(create_request));
        let response_type = match result {
            Ok(response) => ResponseType::EmptyResponse(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        send_response(tx, response_type).await;
    }

    async fn delete_index(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        delete_request: IndexReferenceMessage,
    ) {
        let result = self
            .lockable_db
            .delete_index(tonic::Request::new(delete_request));
        let response_type = match result {
            Ok(response) => ResponseType::EmptyResponse(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        send_response(tx, response_type).await;
    }

    async fn get_index(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        get_request: IndexReferenceMessage,
    ) {
        let result = self.lockable_db.get_index(tonic::Request::new(get_request));
        let response_type = match result {
            Ok(response) => ResponseType::IndexMessage(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        send_response(tx, response_type).await;
    }

    async fn update_index(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        update_request: IndexMessage,
    ) {
        let result = self
            .lockable_db
            .update_index(tonic::Request::new(update_request));
        let response_type = match result {
            Ok(response) => ResponseType::EmptyResponse(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        send_response(tx, response_type).await;
    }
}
