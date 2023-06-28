use crate::ondo_remote::{transaction_response::ResponseType, EmptyMessage, TransactionResponse};
use tonic::Status;

pub(crate) struct EmptyMessageSubServer;

impl EmptyMessageSubServer {
    pub fn process_request(
        &self,
        _tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        _request: EmptyMessage,
    ) -> ResponseType {
        let response_type = ResponseType::EmptyResponse(EmptyMessage {});
        response_type
    }
}
