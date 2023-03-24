use tonic::transport::Server;
use tonic::{Request, Response, Status};

use ondo::ondo_remote;
use ondo_remote::ondo_remote_server::{OndoRemote, OndoRemoteServer};
// use remote::{ArrayOfStringResponse, EmptyMessage, VersionResponse};
use ondo_remote::*;

use ondo::db::server::{
    database_server_trait::DatabaseServerTrait, domain_server_trait::DomainServerTrait,
    index_server_trait::IndexServerTrait, indexed_value_server_trait::IndexedValueServerTrait,
    rocks_db_accessor::RocksDbAccessor, table_server_trait::TableServerTrait,
    table_value_server_trait::TableValueServerTrait,
};

#[derive(Default)]
pub struct MyServer {
    rocks_db_accessor: RocksDbAccessor,
}

#[tonic::async_trait]
impl OndoRemote for MyServer {
    async fn version(&self, r: Request<EmptyMessage>) -> Result<Response<VersionResponse>, Status> {
        self.rocks_db_accessor.version(r)
    }

    async fn create_database_server(
        &self,
        r: Request<DatabaseServerMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        self.rocks_db_accessor.create_database_server(r)
    }

    async fn delete_database_server(
        &self,
        r: Request<DatabaseServerReferenceMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        self.rocks_db_accessor.delete_database_server(r)
    }

    async fn get_database_server(
        &self,
        r: Request<DatabaseServerReferenceMessage>,
    ) -> Result<Response<DatabaseServerMessage>, Status> {
        self.rocks_db_accessor.get_database_server(r)
    }

    async fn update_database_server(
        &self,
        r: Request<DatabaseServerMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        self.rocks_db_accessor.update_database_server(r)
    }

    async fn list_domains(
        &self,
        r: Request<DatabaseServerReferenceMessage>,
    ) -> Result<Response<ArrayOfStringResponse>, Status> {
        self.rocks_db_accessor.list_domains(r)
    }

    async fn create_domain(
        &self,
        r: Request<DomainMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        self.rocks_db_accessor.create_domain(r)
    }

    async fn delete_domain(
        &self,
        r: Request<DomainReferenceMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        self.rocks_db_accessor.delete_domain(r)
    }

    async fn get_domain(
        &self,
        r: Request<DomainReferenceMessage>,
    ) -> Result<Response<DomainMessage>, Status> {
        self.rocks_db_accessor.get_domain(r)
    }

    async fn update_domain(
        &self,
        r: Request<DomainMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        self.rocks_db_accessor.update_domain(r)
    }

    async fn list_tables(
        &self,
        r: Request<DomainReferenceMessage>,
    ) -> Result<Response<ArrayOfStringResponse>, Status> {
        self.rocks_db_accessor.list_tables(r)
    }

    async fn create_table(
        &self,
        r: Request<TableMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        self.rocks_db_accessor.create_table(r)
    }

    async fn delete_table(
        &self,
        r: Request<TableReferenceMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        self.rocks_db_accessor.delete_table(r)
    }

    async fn get_table(
        &self,
        r: Request<TableReferenceMessage>,
    ) -> Result<Response<TableMessage>, Status> {
        self.rocks_db_accessor.get_table(r)
    }

    async fn update_table(
        &self,
        r: Request<TableMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        self.rocks_db_accessor.update_table(r)
    }

    async fn list_indexes(
        &self,
        r: Request<TableReferenceMessage>,
    ) -> Result<Response<ArrayOfStringResponse>, Status> {
        self.rocks_db_accessor.list_indexes(r)
    }

    async fn list_values(
        &self,
        r: Request<TableReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status> {
        self.rocks_db_accessor.list_values(r)
    }

    async fn list_values_by_id_range(
        &self,
        r: Request<TableIdRangeReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status> {
        self.rocks_db_accessor.list_values_by_id_range(r)
    }

    async fn list_values_by_id_list(
        &self,
        r: Request<TableIdListReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status> {
        self.rocks_db_accessor.list_values_by_id_list(r)
    }

    async fn create_index(
        &self,
        r: Request<IndexMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        self.rocks_db_accessor.create_index(r)
    }

    async fn delete_index(
        &self,
        r: Request<IndexReferenceMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        self.rocks_db_accessor.delete_index(r)
    }

    async fn get_index(
        &self,
        r: Request<IndexReferenceMessage>,
    ) -> Result<Response<IndexMessage>, Status> {
        self.rocks_db_accessor.get_index(r)
    }

    async fn update_index(
        &self,
        r: Request<IndexMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        self.rocks_db_accessor.update_index(r)
    }

    async fn create_value(
        &self,
        r: Request<CreateTableValueMessage>,
    ) -> Result<Response<OndoKeyMessage>, Status> {
        self.rocks_db_accessor.create_value(r)
    }

    async fn delete_value(
        &self,
        r: Request<TableValueReferenceMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        self.rocks_db_accessor.delete_value(r)
    }

    async fn get_value(
        &self,
        r: Request<TableValueReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status> {
        self.rocks_db_accessor.get_value(r)
    }

    async fn update_value(
        &self,
        r: Request<TableValueMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        self.rocks_db_accessor.update_value(r)
    }

    async fn find_values(
        &self,
        r: Request<IndexedValueReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status> {
        self.rocks_db_accessor.find_values(r)
    }

    async fn find_values_by_range(
        &self,
        r: Request<IndexedValueRangeReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status> {
        self.rocks_db_accessor.find_values_by_range(r)
    }
}

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
