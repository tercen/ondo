// FIXME: For all sub-servers, all iterators should directly write to the stream. Currently they are collecting a vector
use futures::Stream;
use std::pin::Pin;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;

use super::send_response::{blocking_send_response, send_response};

use crate::db::server::lockable_db::transaction_maker::LockableTransactionOrDb;
use crate::db::server::lockable_db::LOCKABLE_DB;
use crate::ondo_remote;
use ondo_remote::*;
use parking_lot::ReentrantMutex;
use std::sync::Arc;

use super::my_server::MyServer; // request and response messages

#[tonic::async_trait]
impl ondo_remote_server::OndoRemote for MyServer {
    type TransactionStreamStream = Pin<
        Box<dyn Stream<Item = Result<TransactionResponse, tonic::Status>> + Send + Sync + 'static>,
    >;
    type DbStreamStream = Pin<
        Box<dyn Stream<Item = Result<TransactionResponse, tonic::Status>> + Send + Sync + 'static>,
    >;
    type MetaStreamStream = Pin<
        Box<dyn Stream<Item = Result<TransactionResponse, tonic::Status>> + Send + Sync + 'static>,
    >;

    async fn transaction_stream(
        &self,
        request: tonic::Request<tonic::Streaming<TransactionRequest>>,
    ) -> Result<tonic::Response<Self::TransactionStreamStream>, tonic::Status> {
        let (answer_tx, answer_rx) = mpsc::channel(16);
        let (client_tx, client_rx) = mpsc::channel(16);

        let mut stream = request.into_inner();
        let my_server_clone = self.clone();
        tokio::spawn(async move {
            let _ = tokio::task::spawn_blocking(move || {
                transaction_loop(my_server_clone, client_rx, answer_tx.clone());
            });
            tokio::task::yield_now().await;
            while let Some(request) = stream.next().await {
                tokio::task::yield_now().await;
                client_tx.send(request).await;
            }
        });

        let response_stream = ReceiverStream::new(answer_rx);

        Ok(tonic::Response::new(
            Box::pin(response_stream) as Self::TransactionStreamStream
        ))
    }

    async fn db_stream(
        &self,
        request: tonic::Request<tonic::Streaming<TransactionRequest>>,
    ) -> Result<tonic::Response<Self::TransactionStreamStream>, tonic::Status> {
        let (tx, rx) = mpsc::channel(16);

        let mut stream = request.into_inner();

        let my_server_clone = self.clone();
        tokio::spawn(async move {
            let lockable_db = LockableTransactionOrDb::with_db(LOCKABLE_DB.clone());
            // let lockable_db = LockableTransactionOrDb::with_db(LockableDb::in_memory());
            let lockable_transaction_or_db = lockable_db;
            while let Some(request) = stream.next().await {
                match request {
                    Ok(transaction_request) => {
                        match transaction_request.request_type {
                            Some(transaction_request::RequestType::TableValueOps(
                                table_value_ops,
                            )) => {
                                let response_type = my_server_clone
                                    .table_value_ops_sub_server(lockable_transaction_or_db.clone())
                                    .process_request(
                                        tx.clone(),
                                        table_value_ops.request_type.unwrap(),
                                    );
                                send_response(tx.clone(), response_type);
                            }
                            Some(transaction_request::RequestType::IndexedValueOps(
                                indexed_value_ops,
                            )) => {
                                let response_type = my_server_clone
                                    .indexed_value_ops_sub_server(
                                        lockable_transaction_or_db.clone(),
                                    )
                                    .process_request(
                                        tx.clone(),
                                        indexed_value_ops.request_type.unwrap(),
                                    );
                                send_response(tx.clone(), response_type);
                            }
                            Some(transaction_request::RequestType::KeyPrefixOps(
                                key_prefix_ops,
                            )) => {
                                let response_type = my_server_clone
                                    .key_prefix_ops_sub_server(lockable_transaction_or_db.clone())
                                    .process_request(
                                        tx.clone(),
                                        key_prefix_ops.request_type.unwrap(),
                                    );
                                send_response(tx.clone(), response_type);
                            }
                            None => {
                                // You could return an error here if you want
                            }
                        }
                    }
                    Err(err) => {
                        // Handle stream error
                        eprintln!("Error receiving request: {:?}", err);
                        break;
                    }
                }
            }
        });

        let response_stream = ReceiverStream::new(rx);

        Ok(tonic::Response::new(
            Box::pin(response_stream) as Self::TransactionStreamStream
        ))
    }

    async fn meta_stream(
        &self,
        request: tonic::Request<tonic::Streaming<MetaRequest>>,
    ) -> Result<tonic::Response<Self::TransactionStreamStream>, tonic::Status> {
        let (tx, rx) = mpsc::channel(16);

        let mut stream = request.into_inner();

        //FIXME: Use database but do atomic writes
        let my_server_clone = self.clone();
        tokio::spawn(async move {
            let lockable_db = LockableTransactionOrDb::with_db(LOCKABLE_DB.clone());
            // let lockable_db = LockableTransactionOrDb::with_db(LockableDb::in_memory());
            let lockable_transaction_or_db = lockable_db;
            while let Some(request) = stream.next().await {
                match request {
                    Ok(meta_request) => {
                        match meta_request.request_type {
                            Some(meta_request::RequestType::VersionRequest(version_request)) => {
                                let response_type = my_server_clone
                                    .version_sub_server(lockable_transaction_or_db.clone())
                                    .process_request(tx.clone(), version_request);
                                send_response(tx.clone(), response_type);
                            }
                            Some(meta_request::RequestType::DatabaseServerOps(
                                database_server_ops,
                            )) => {
                                let response_type = my_server_clone
                                    .database_server_ops_sub_server(
                                        lockable_transaction_or_db.clone(),
                                    )
                                    .process_request(
                                        tx.clone(),
                                        database_server_ops.request_type.unwrap(),
                                    );
                                send_response(tx.clone(), response_type);
                            }
                            Some(meta_request::RequestType::DomainOps(domain_ops)) => {
                                let response_type = my_server_clone
                                    .domain_ops_sub_server(lockable_transaction_or_db.clone())
                                    .process_request(tx.clone(), domain_ops.request_type.unwrap());
                                send_response(tx.clone(), response_type);
                            }
                            Some(meta_request::RequestType::TableOps(table_ops)) => {
                                let response_type = my_server_clone
                                    .table_ops_sub_server(lockable_transaction_or_db.clone())
                                    .process_request(tx.clone(), table_ops.request_type.unwrap());
                                send_response(tx.clone(), response_type);
                            }
                            Some(meta_request::RequestType::IndexOps(index_ops)) => {
                                let response_type = my_server_clone
                                    .index_ops_sub_server(lockable_transaction_or_db.clone())
                                    .process_request(tx.clone(), index_ops.request_type.unwrap());
                                send_response(tx.clone(), response_type);
                            }
                            Some(meta_request::RequestType::TextIndexOps(text_index_ops)) => {
                                let response_type = my_server_clone
                                    .text_index_ops_sub_server(lockable_transaction_or_db.clone())
                                    .process_request(
                                        tx.clone(),
                                        text_index_ops.request_type.unwrap(),
                                    );
                                send_response(tx.clone(), response_type);
                            }
                            None => {
                                // You could return an error here if you want
                            }
                        }
                    }
                    Err(err) => {
                        // Handle stream error
                        eprintln!("Error receiving request: {:?}", err);
                        break;
                    }
                }
            }
        });

        let response_stream = ReceiverStream::new(rx);

        Ok(tonic::Response::new(
            Box::pin(response_stream) as Self::TransactionStreamStream
        ))
    }
}

//FIXME: add transaction commit and abort
fn transaction_loop(
    my_server_clone: MyServer,
    mut client_rx: tokio::sync::mpsc::Receiver<Result<TransactionRequest, tonic::Status>>,
    answer_tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, tonic::Status>>,
) {
    let lockable_db = LOCKABLE_DB.clone();
    let db_guard = lockable_db.read();
    let transaction = db_guard.transaction();

    let lockable_transaction = LockableTransactionOrDb {
        transaction: Some(Arc::new(ReentrantMutex::new(transaction))),
        lockable_db: lockable_db.clone(),
    };

    while let Some(request) = client_rx.blocking_recv() {
        match request {
            Ok(transaction_request) => {
                match transaction_request.request_type {
                    Some(transaction_request::RequestType::TableValueOps(table_value_ops)) => {
                        let response_type = my_server_clone
                            .table_value_ops_sub_server(lockable_transaction.clone())
                            .process_request(
                                answer_tx.clone(),
                                table_value_ops.request_type.unwrap(),
                            );
                        blocking_send_response(answer_tx.clone(), response_type);
                    }
                    Some(transaction_request::RequestType::IndexedValueOps(indexed_value_ops)) => {
                        let response_type = my_server_clone
                            .indexed_value_ops_sub_server(lockable_transaction.clone())
                            .process_request(
                                answer_tx.clone(),
                                indexed_value_ops.request_type.unwrap(),
                            );
                        blocking_send_response(answer_tx.clone(), response_type);
                    }
                    Some(transaction_request::RequestType::KeyPrefixOps(key_prefix_ops)) => {
                        let response_type = my_server_clone
                            .key_prefix_ops_sub_server(lockable_transaction.clone())
                            .process_request(
                                answer_tx.clone(),
                                key_prefix_ops.request_type.unwrap(),
                            );
                        blocking_send_response(answer_tx.clone(), response_type);
                    }
                    None => {
                        // You could return an error here if you want
                    }
                }
            }
            Err(err) => {
                // Handle stream error
                eprintln!("Error receiving request: {:?}", err);
                break;
            }
        }
    }
}
