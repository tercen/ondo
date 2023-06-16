use crate::db::server::index_server_trait::IndexServerTrait;
use crate::db::server::lockable_db::transaction_maker::LockableTransactionOrDb;
use crate::ondo_remote::{
    indexed_value_ops::RequestType, transaction_response::ResponseType,
    IndexedValueRangeReferenceMessage, IndexedValueReferenceMessage, TransactionResponse,
};
use tonic::Status;

pub(crate) struct IndexedValueOpsSubServer<'a> {
    pub lockable_db: LockableTransactionOrDb<'a>,
}

impl<'a> IndexedValueOpsSubServer<'a> {
    pub fn process_request(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        request: RequestType,
    ) -> ResponseType {
        match request {
            RequestType::FindValues(get_request) => self.find_values(tx, get_request),
            RequestType::FindValuesByRange(list_request) => {
                self.find_values_by_range(tx, list_request)
            }
        }
    }

    fn find_values(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        get_request: IndexedValueReferenceMessage,
    ) -> ResponseType {
        let result = self
            .lockable_db
            .find_values(tonic::Request::new(get_request));
        let response_type = match result {
            Ok(response) => ResponseType::JsonMessage(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        response_type
    }

    fn find_values_by_range(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        list_request: IndexedValueRangeReferenceMessage,
    ) -> ResponseType {
        let result = self
            .lockable_db
            .find_values_by_range(tonic::Request::new(list_request));
        let response_type = match result {
            Ok(response) => ResponseType::JsonMessage(response.into_inner()),
            Err(status) => ResponseType::ErrorResponse(status.into()),
        };
        response_type
    }
}
