use crate::ondo_remote::{transaction_response::ResponseType, TransactionResponse};
use tonic::Status;

pub(super) async fn send_response(
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

pub(super) async fn send_status_response(
    tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
    status: Status,
) {
    if let Err(err) = tx.send(Err(status)).await {
        eprintln!("Error sending response: {:?}", err);
    }
}

pub(super) fn blocking_send_response(
    tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
    response_type: ResponseType,
) {
    let response = TransactionResponse {
        response_type: Some(response_type),
    };

    if let Err(err) = tx.blocking_send(Ok(response)) {
        eprintln!("Error sending response: {:?}", err);
    }
}

pub(super) fn blocking_send_status_response(
    tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
    status: Status,
) {
    if let Err(err) = tx.blocking_send(Err(status)) {
        eprintln!("Error sending response: {:?}", err);
    }
}
