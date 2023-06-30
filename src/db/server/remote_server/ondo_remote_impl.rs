// FIXME: For all sub-servers, all iterators should directly write to the stream. Currently they are collecting a vector
use futures::Stream;
use std::pin::Pin;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;

use super::send_response::send_response;

use crate::db::server::lockable_db::transaction_or_db::TransactionOrDb;
use crate::db::server::lockable_db::LOCKABLE_DB;
use crate::ondo_remote;
use ondo_remote::*;

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

        let mut stream = request.into_inner();
        let my_server_clone = self.clone();
        tokio::spawn(async move {
            let lockable_db = LOCKABLE_DB.clone();
            let db_guard = lockable_db.read().await;
            let db = &*db_guard;
            let transaction = db.transaction();

            let mut network_error_occurred = false;

            while let Some(request) = stream.next().await {
                match request {
                    Ok(transaction_request) => {
                        match transaction_request.request_type {
                            Some(transaction_request::RequestType::TableValueOps(
                                table_value_ops,
                            )) => {
                                let transaction_or_db =
                                    TransactionOrDb::Transaction(&transaction, db);
                                let response_type = my_server_clone
                                    .table_value_ops_sub_server(transaction_or_db)
                                    .process_request(
                                        answer_tx.clone(),
                                        table_value_ops.request_type.unwrap(),
                                    );
                                send_response(answer_tx.clone(), response_type).await;
                            }
                            Some(transaction_request::RequestType::IndexedValueOps(
                                indexed_value_ops,
                            )) => {
                                let transaction_or_db =
                                    TransactionOrDb::Transaction(&transaction, db);
                                let response_type = my_server_clone
                                    .indexed_value_ops_sub_server(transaction_or_db)
                                    .process_request(
                                        answer_tx.clone(),
                                        indexed_value_ops.request_type.unwrap(),
                                    );
                                send_response(answer_tx.clone(), response_type).await;
                            }
                            Some(transaction_request::RequestType::KeyPrefixOps(
                                key_prefix_ops,
                            )) => {
                                let transaction_or_db =
                                    TransactionOrDb::Transaction(&transaction, db);
                                let response_type = my_server_clone
                                    .tabled_value_ops_sub_server(transaction_or_db)
                                    .process_request(
                                        answer_tx.clone(),
                                        key_prefix_ops.request_type.unwrap(),
                                    );
                                send_response(answer_tx.clone(), response_type).await;
                            }
                            None => {
                                // You could return an error here if you want
                            }
                        }
                    }
                    Err(err) => {
                        // Handle stream error
                        eprintln!("Error receiving request: {:?}", err);
                        network_error_occurred = true;
                        break;
                    }
                }
            }
            if !network_error_occurred {
                let db_result = transaction.commit();
                match db_result {
                    Ok(_) => {
                        // Do nothing
                    }
                    Err(err) => {
                        let db_err = crate::db::db_error::DbError::RocksDbError(err);
                        let status: tonic::Status =
                            crate::db::server::db_error_to_status::db_error_to_status(db_err);
                        super::send_response::send_status_response(answer_tx.clone(), status).await;
                    }
                }
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
            let lockable_db = LOCKABLE_DB.clone();
            let db_guard = lockable_db.read().await;
            let db = &*db_guard;
            while let Some(request) = stream.next().await {
                match request {
                    Ok(transaction_request) => {
                        match transaction_request.request_type {
                            Some(transaction_request::RequestType::TableValueOps(
                                table_value_ops,
                            )) => {
                                let transaction_or_db = TransactionOrDb::Db(db);
                                let response_type = my_server_clone
                                    .table_value_ops_sub_server(transaction_or_db)
                                    .process_request(
                                        tx.clone(),
                                        table_value_ops.request_type.unwrap(),
                                    );
                                send_response(tx.clone(), response_type).await;
                            }
                            Some(transaction_request::RequestType::IndexedValueOps(
                                indexed_value_ops,
                            )) => {
                                let transaction_or_db = TransactionOrDb::Db(db);
                                let response_type = my_server_clone
                                    .indexed_value_ops_sub_server(transaction_or_db)
                                    .process_request(
                                        tx.clone(),
                                        indexed_value_ops.request_type.unwrap(),
                                    );
                                send_response(tx.clone(), response_type).await;
                            }
                            Some(transaction_request::RequestType::KeyPrefixOps(
                                key_prefix_ops,
                            )) => {
                                let transaction_or_db = TransactionOrDb::Db(db);
                                let response_type = my_server_clone
                                    .tabled_value_ops_sub_server(transaction_or_db)
                                    .process_request(
                                        tx.clone(),
                                        key_prefix_ops.request_type.unwrap(),
                                    );
                                send_response(tx.clone(), response_type).await;
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
            let lockable_db = LOCKABLE_DB.clone();
            let mut db_guard = lockable_db.write().await;
            let db = &mut *db_guard;
            while let Some(request) = stream.next().await {
                match request {
                    Ok(meta_request) => {
                        match meta_request.request_type {
                            Some(meta_request::RequestType::VersionRequest(version_request)) => {
                                let transaction_or_db = TransactionOrDb::Db(db);
                                let response_type = my_server_clone
                                    .version_sub_server(transaction_or_db)
                                    .process_request(tx.clone(), version_request);
                                send_response(tx.clone(), response_type).await;
                            }
                            Some(meta_request::RequestType::DatabaseServerOps(
                                database_server_ops,
                            )) => {
                                let response_type = my_server_clone
                                    .database_server_ops_sub_server(db)
                                    .process_request(
                                        tx.clone(),
                                        database_server_ops.request_type.unwrap(),
                                    );
                                send_response(tx.clone(), response_type).await;
                            }
                            Some(meta_request::RequestType::DomainOps(domain_ops)) => {
                                let response_type = my_server_clone
                                    .domain_ops_sub_server(db)
                                    .process_request(tx.clone(), domain_ops.request_type.unwrap());
                                send_response(tx.clone(), response_type).await;
                            }
                            Some(meta_request::RequestType::TableOps(table_ops)) => {
                                let response_type = my_server_clone
                                    .table_ops_sub_server(db)
                                    .process_request(tx.clone(), table_ops.request_type.unwrap());
                                send_response(tx.clone(), response_type).await;
                            }
                            Some(meta_request::RequestType::IndexOps(index_ops)) => {
                                let response_type = my_server_clone
                                    .index_ops_sub_server(db)
                                    .process_request(tx.clone(), index_ops.request_type.unwrap());
                                send_response(tx.clone(), response_type).await;
                            }
                            Some(meta_request::RequestType::TextIndexOps(text_index_ops)) => {
                                let transaction_or_db = TransactionOrDb::Db(db);
                                let response_type = my_server_clone
                                    .text_index_ops_sub_server(transaction_or_db)
                                    .process_request(
                                        tx.clone(),
                                        text_index_ops.request_type.unwrap(),
                                        lockable_db.clone(),
                                    );
                                send_response(tx.clone(), response_type).await;
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
