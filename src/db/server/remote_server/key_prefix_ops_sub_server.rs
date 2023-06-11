use super::send_response::send_response;
use crate::db::server::lockable_db::transaction_maker::TransactionMaker;
use crate::db::server::table_server_trait::TableServerTrait;
use crate::ondo_remote::{
    key_prefix_ops::RequestType, transaction_response::ResponseType, TableIdListReferenceMessage,
    TableIdRangeReferenceMessage, TableReferenceMessage, TableValueReferenceMessage,
    TransactionResponse,
};
use tonic::Status;

pub(crate) struct KeyPrefixOpsSubServer<'a> {
    pub lockable_db: TransactionMaker<'a>,
}

impl<'a> KeyPrefixOpsSubServer<'a> {
    pub async fn process_request(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        request: RequestType,
    ) {
        match request {
            RequestType::ListValuesRequest(list_request) => {
                self.list_values(tx, list_request).await;
            }
            RequestType::ListValuesByKeyPrefixRequest(list_by_prefix_request) => {
                self.list_values_by_key_prefix(tx, list_by_prefix_request)
                    .await;
            }
            RequestType::ListValuesByIdRangeRequest(list_by_id_range_request) => {
                self.list_values_by_id_range(tx, list_by_id_range_request)
                    .await;
            }
            RequestType::ListValuesByIdListRequest(list_by_id_list_request) => {
                self.list_values_by_id_list(tx, list_by_id_list_request)
                    .await;
            }
        }
    }

    async fn list_values(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        list_values_request: TableReferenceMessage,
    ) {
        let result = self
            .lockable_db
            .list_values(tonic::Request::new(list_values_request));
        let response_type = match result {
            Ok(response) => ResponseType::JsonMessage(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status),
        };
        send_response(tx, response_type).await;
    }

    async fn list_values_by_key_prefix(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        list_values_request: TableValueReferenceMessage,
    ) {
        let result = self
            .lockable_db
            .list_values_by_key_prefix(tonic::Request::new(list_values_request));
        let response_type = match result {
            Ok(response) => ResponseType::JsonMessage(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status),
        };
        send_response(tx, response_type).await;
    }

    async fn list_values_by_id_range(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        list_values_request: TableIdRangeReferenceMessage,
    ) {
        let result = self
            .lockable_db
            .list_values_by_id_range(tonic::Request::new(list_values_request));
        let response_type = match result {
            Ok(response) => ResponseType::JsonMessage(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status),
        };
        send_response(tx, response_type).await;
    }

    // FIXME: Deep down rocks db has a method to get multiple values using threads. We are not using it here.
    async fn list_values_by_id_list(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        list_values_request: TableIdListReferenceMessage,
    ) {
        let result = self
            .lockable_db
            .list_values_by_id_list(tonic::Request::new(list_values_request));
        let response_type = ResponseType::JsonMessage(Default::default());
        send_response(tx, response_type).await;
    }
}
