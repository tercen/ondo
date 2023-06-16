use crate::db::server::lockable_db::transaction_maker::LockableTransactionOrDb;
use crate::db::server::table_server_trait::TableServerTrait;
use crate::ondo_remote::{
    table_ops::RequestType, transaction_response::ResponseType, TableMessage,
    TableReferenceMessage, TransactionResponse,
};
use tonic::Status;

pub(crate) struct TableOpsSubServer<'a> {
    pub lockable_db: LockableTransactionOrDb<'a>,
}

impl<'a> TableOpsSubServer<'a> {
    pub fn process_request(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        request: RequestType,
    ) -> ResponseType {
        match request {
            RequestType::CreateRequest(create_request) => self.create_table(tx, create_request),
            RequestType::DeleteRequest(delete_request) => self.delete_table(tx, delete_request),
            RequestType::GetRequest(get_request) => self.get_table(tx, get_request),
            RequestType::UpdateRequest(update_request) => self.update_table(tx, update_request),
            RequestType::ListIndexesRequest(list_indexes_request) => {
                self.list_indexes(tx, list_indexes_request)
            }
        }
    }

    fn create_table(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        create_request: TableMessage,
    ) -> ResponseType {
        let result = self
            .lockable_db
            .create_table(tonic::Request::new(create_request));
        let response_type = match result {
            Ok(response) => ResponseType::EmptyResponse(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        response_type
    }

    fn delete_table(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        delete_request: TableReferenceMessage,
    ) -> ResponseType {
        let result = self
            .lockable_db
            .delete_table(tonic::Request::new(delete_request));
        let response_type = match result {
            Ok(response) => ResponseType::EmptyResponse(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        response_type
    }

    fn get_table(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        get_request: TableReferenceMessage,
    ) -> ResponseType {
        let result = self.lockable_db.get_table(tonic::Request::new(get_request));
        let response_type = match result {
            Ok(response) => ResponseType::TableMessage(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        response_type
    }

    fn update_table(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        update_request: TableMessage,
    ) -> ResponseType {
        let result = self
            .lockable_db
            .update_table(tonic::Request::new(update_request));
        let response_type = match result {
            Ok(response) => ResponseType::EmptyResponse(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        response_type
    }

    fn list_indexes(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        list_indexes_request: TableReferenceMessage,
    ) -> ResponseType {
        let result = self
            .lockable_db
            .list_indexes(tonic::Request::new(list_indexes_request));
        let response_type = match result {
            Ok(response) => ResponseType::ArrayOfStringResponse(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        response_type
    }
}
