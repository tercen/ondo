use crate::ondo_remote::{transaction_response::ResponseType, TransactionResponse};
use tonic::Status;

pub(super) fn send_response(
    tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
    response_type: ResponseType,
) {
    let response = TransactionResponse {
        response_type: Some(response_type),
    };

    if let Err(err) = tx.send(Ok(response)).await {
        eprintln!("Error sending response: {:?}", err);
    }
}

pub(super) fn send_status_response(
    tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
    status: Status,
) {
    if let Err(err) = tx.send(Err(status)).await {
        eprintln!("Error sending response: {:?}", err);
    }
}
