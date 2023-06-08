//FIXME. Do not fetch the key from the record on Update. Just use the key from the request.
use super::send_response::send_response;
use crate::db::server::lockable_db::transaction_maker::TransactionMaker;
use crate::db::server::table_value_server_trait::TableValueServerTrait;
use crate::ondo_remote::{
    table_value_ops::RequestType, transaction_response::ResponseType, CreateTableValueMessage,
    TableValueMessage, TableValueReferenceMessage, TransactionResponse,
};
use tonic::Status;

pub(crate) struct TableValueOpsSubServer<'a> {
    pub lockable_db: TransactionMaker<'a>,
}

impl<'a> TableValueOpsSubServer<'a> {
    pub async fn process_request(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        request: RequestType,
    ) {
        match request {
            RequestType::CreateRequest(create_request) => {
                self.create_table_value(tx, create_request).await;
            }
            RequestType::DeleteRequest(delete_request) => {
                self.delete_table_value(tx, delete_request).await;
            }
            RequestType::GetRequest(get_request) => {
                self.get_table_value(tx, get_request).await;
            }
            RequestType::UpdateRequest(update_request) => {
                self.update_table_value(tx, update_request).await;
            }
            RequestType::GetForUpdateRequest(get_request) => {
                self.get_table_value_for_update(tx, get_request).await;
            }
        }
    }

    async fn create_table_value(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        create_request: CreateTableValueMessage,
    ) {
        let result = self
            .lockable_db
            .create_value(tonic::Request::new(create_request));
        let response_type = match result {
            Ok(response) => ResponseType::OndoKeyMessage(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status),
        };
        send_response(tx, response_type).await;
    }

    async fn delete_table_value(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        delete_request: TableValueReferenceMessage,
    ) {
        let result = self
            .lockable_db
            .delete_value(tonic::Request::new(delete_request));
        let response_type = match result {
            Ok(response) => ResponseType::EmptyResponse(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status),
        };
        send_response(tx, response_type).await;
    }

    async fn get_table_value(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        get_request: TableValueReferenceMessage,
    ) {
        let result = self.lockable_db.get_value(tonic::Request::new(get_request));
        let response_type = match result {
            Ok(response) => ResponseType::JsonMessage(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status),
        };
        send_response(tx, response_type).await;
    }

    async fn update_table_value(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        update_request: TableValueMessage,
    ) {
        let result = self
            .lockable_db
            .update_value(tonic::Request::new(update_request));
        let response_type = match result {
            Ok(response) => ResponseType::EmptyResponse(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status),
        };
        send_response(tx, response_type).await;
    }

    async fn get_table_value_for_update(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        get_request: TableValueReferenceMessage,
    ) {
        let result = self.lockable_db.get_value_for_update(tonic::Request::new(get_request));
        let response_type = match result {
            Ok(response) => ResponseType::JsonMessage(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status),
        };
        send_response(tx, response_type).await;
    }

}
