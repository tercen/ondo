use crate::db::server::lockable_db::transaction_maker::LockableTransactionOrDb;
use crate::db::server::table_server_trait::TableServerTrait;
use crate::ondo_remote::{
    key_prefix_ops::RequestType, transaction_response::ResponseType, TableIdListReferenceMessage,
    TableIdRangeReferenceMessage, TableReferenceMessage, TableValueReferenceMessage,
    TransactionResponse,
};
use tonic::Status;

pub(crate) struct KeyPrefixOpsSubServer<'a> {
    pub lockable_db: LockableTransactionOrDb<'a>,
}

impl<'a> KeyPrefixOpsSubServer<'a> {
    pub fn process_request(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        request: RequestType,
    ) -> ResponseType {
        match request {
            RequestType::ListValuesRequest(list_request) => self.list_values(tx, list_request),
            RequestType::ListValuesByKeyPrefixRequest(list_by_prefix_request) => {
                self.list_values_by_key_prefix(tx, list_by_prefix_request)
            }
            RequestType::ListValuesByIdRangeRequest(list_by_id_range_request) => {
                self.list_values_by_id_range(tx, list_by_id_range_request)
            }
            RequestType::ListValuesByIdListRequest(list_by_id_list_request) => {
                self.list_values_by_id_list(tx, list_by_id_list_request)
            }
        }
    }

    fn list_values(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        list_values_request: TableReferenceMessage,
    ) -> ResponseType {
        let result = self
            .lockable_db
            .list_values(tonic::Request::new(list_values_request));
        let response_type = match result {
            Ok(response) => ResponseType::JsonMessage(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        response_type
    }

    fn list_values_by_key_prefix(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        list_values_request: TableValueReferenceMessage,
    ) -> ResponseType {
        let result = self
            .lockable_db
            .list_values_by_key_prefix(tonic::Request::new(list_values_request));
        let response_type = match result {
            Ok(response) => ResponseType::JsonMessage(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        response_type
    }

    fn list_values_by_id_range(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        list_values_request: TableIdRangeReferenceMessage,
    ) -> ResponseType {
        let result = self
            .lockable_db
            .list_values_by_id_range(tonic::Request::new(list_values_request));
        let response_type = match result {
            Ok(response) => ResponseType::JsonMessage(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        response_type
    }

    // FIXME: Deep down rocks db has a method to get multiple values using threads. We are not using it here.
    fn list_values_by_id_list(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        list_values_request: TableIdListReferenceMessage,
    ) -> ResponseType {
        let result = self
            .lockable_db
            .list_values_by_id_list(tonic::Request::new(list_values_request));
        let response_type = ResponseType::JsonMessage(Default::default());
        response_type
    }
}
