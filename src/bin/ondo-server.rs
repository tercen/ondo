use std::pin::Pin;

use futures::Stream;
use tokio::sync::mpsc;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use ondo::ondo_remote;
use ondo_remote::ondo_remote_server::{OndoRemote, OndoRemoteServer};
use ondo_remote::*;
use ondo::db::remote_server::my_server::MyServer;

//FIXME: Launch Task Queue and Make sure Task Queue Uses TransactionDB
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        std::process::exit(0);
    });

    let addr = "0.0.0.0:50051".parse()?;

    let remote_server = MyServer::default();
    Server::builder()
        .add_service(OndoRemoteServer::new(remote_server))
        .serve(addr)
        .await?;

    Ok(())
}
