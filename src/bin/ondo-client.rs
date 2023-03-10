use tonic::transport::Endpoint;
use tonic::Request;

use ondo::remote;
use remote::remote_client::RemoteClient;
use remote::*; // request and response messages

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = Endpoint::from_static("https://127.0.0.1:50051");
    
    let mut client  = RemoteClient::connect(addr).await?;
    let request = Request::new(EmptyMessage{});
    let response = client.version(request).await?;
    println!("response: {}", response.into_inner().version);

    Ok(())
}