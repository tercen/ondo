//FIXME. Do not fetch the key from the record on Update. Just use the key from the request.

use crate::db::server::lockable_db::transaction_or_db::TransactionOrDb;
use crate::db::server::table_value_server_trait::TableValueServerTrait;
use crate::ondo_remote::{
    table_value_ops::RequestType, transaction_response::ResponseType, CreateTableValueMessage,
    TableValueMessage, TableValueReferenceMessage, TransactionResponse,
};
use tonic::Status;

pub(crate) struct TableValueOpsSubServer<'a> {
    pub transaction_or_db: TransactionOrDb<'a>,
}

impl<'a> TableValueOpsSubServer<'a> {
    pub fn process_request(
        &self,
        _tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        request: RequestType,
    ) -> ResponseType {
        match request {
            RequestType::CreateRequest(create_request) => {
                self.create_table_value(_tx, create_request)
            }
            RequestType::DeleteRequest(delete_request) => {
                self.delete_table_value(_tx, delete_request)
            }
            RequestType::GetRequest(get_request) => self.get_table_value(_tx, get_request),
            RequestType::UpdateRequest(update_request) => {
                self.update_table_value(_tx, update_request)
            }
            RequestType::GetForUpdateRequest(get_request) => {
                self.get_table_value_for_update(_tx, get_request)
            }
        }
    }

    fn create_table_value(
        &self,
        _tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        create_request: CreateTableValueMessage,
    ) -> ResponseType {
        let result = self
            .transaction_or_db
            .create_value(tonic::Request::new(create_request));
        let response_type = match result {
            Ok(response) => ResponseType::OndoKeyMessage(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        response_type
    }

    fn delete_table_value(
        &self,
        _tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        delete_request: TableValueReferenceMessage,
    ) -> ResponseType {
        let result = self
            .transaction_or_db
            .delete_value(tonic::Request::new(delete_request));
        let response_type = match result {
            Ok(response) => ResponseType::EmptyResponse(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        response_type
    }

    fn get_table_value(
        &self,
        _tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        get_request: TableValueReferenceMessage,
    ) -> ResponseType {
        let result = self
            .transaction_or_db
            .get_value(tonic::Request::new(get_request));
        let response_type = match result {
            Ok(response) => ResponseType::JsonMessage(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        response_type
    }

    fn update_table_value(
        &self,
        _tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        update_request: TableValueMessage,
    ) -> ResponseType {
        let result = self
            .transaction_or_db
            .update_value(tonic::Request::new(update_request));
        let response_type = match result {
            Ok(response) => ResponseType::EmptyResponse(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        response_type
    }

    fn get_table_value_for_update(
        &self,
        _tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        get_request: TableValueReferenceMessage,
    ) -> ResponseType {
        let result = self
            .transaction_or_db
            .get_value_for_update(tonic::Request::new(get_request));
        let response_type = match result {
            Ok(response) => ResponseType::JsonMessage(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        response_type
    }
}
