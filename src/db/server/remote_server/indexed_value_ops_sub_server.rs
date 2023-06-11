use super::send_response::send_response;
use crate::db::server::index_server_trait::IndexServerTrait;
use crate::db::server::lockable_db::transaction_maker::TransactionMaker;
use crate::ondo_remote::{
    indexed_value_ops::RequestType, transaction_response::ResponseType,
    IndexedValueRangeReferenceMessage, IndexedValueReferenceMessage, TransactionResponse,
};
use tonic::Status;

pub(crate) struct IndexedValueOpsSubServer<'a> {
    pub lockable_db: TransactionMaker<'a>,
}

impl<'a> IndexedValueOpsSubServer<'a> {
    pub async fn process_request(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        request: RequestType,
    ) {
        match request {
            RequestType::FindValues(get_request) => {
                self.find_values(tx, get_request).await;
            }
            RequestType::FindValuesByRange(list_request) => {
                self.find_values_by_range(tx, list_request).await;
            }
        }
    }

    async fn find_values(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        get_request: IndexedValueReferenceMessage,
    ) {
        let result = self
            .lockable_db
            .find_values(tonic::Request::new(get_request));
        let response_type = match result {
            Ok(response) => ResponseType::JsonMessage(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        send_response(tx, response_type);
    }

    async fn find_values_by_range(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        list_request: IndexedValueRangeReferenceMessage,
    ) {
        let result = self
            .lockable_db
            .find_values_by_range(tonic::Request::new(list_request));
        let response_type = match result {
            Ok(response) => ResponseType::JsonMessage(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        send_response(tx, response_type);
    }
}
