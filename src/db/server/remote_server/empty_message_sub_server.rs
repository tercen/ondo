use super::send_response::send_response;
use crate::ondo_remote::{transaction_response::ResponseType, EmptyMessage, TransactionResponse};
use tonic::Status;

pub(crate) struct EmptyMessageSubServer;

impl EmptyMessageSubServer {
    pub async fn process_request(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        _request: EmptyMessage,
    ) {
        let response_type = ResponseType::EmptyResponse(EmptyMessage {});
        send_response(tx, response_type);
    }
}
