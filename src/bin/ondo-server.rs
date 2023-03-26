use tonic::transport::Server;
use tonic::{Request, Response, Status};

use ondo::ondo_remote;
use ondo_remote::ondo_remote_server::{OndoRemote, OndoRemoteServer};
use ondo_remote::*;

use ondo::db::server::{
    database_server_trait::DatabaseServerTrait, domain_server_trait::DomainServerTrait,
    index_server_trait::IndexServerTrait, rocks_db_accessor::RocksDbAccessor,
    table_server_trait::TableServerTrait, table_value_server_trait::TableValueServerTrait,
};

#[derive(Default)]
pub struct MyServer {
    rocks_db_accessor: RocksDbAccessor,
}

#[tonic::async_trait]
impl OndoRemote for MyServer {
    /// Returns the version of the server.
    async fn version(&self, r: Request<EmptyMessage>) -> Result<Response<VersionResponse>, Status> {
        self.rocks_db_accessor.version(r)
    }

    /// Creates a new database server with the given configuration.
    async fn create_database_server(
        &self,
        r: Request<DatabaseServerMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        self.rocks_db_accessor.create_database_server(r)
    }

    /// Deletes an existing database server identified by the given reference.
    async fn delete_database_server(
        &self,
        r: Request<DatabaseServerReferenceMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        self.rocks_db_accessor.delete_database_server(r)
    }

    /// Retrieves the configuration of an existing database server identified by the given reference.
    async fn get_database_server(
        &self,
        r: Request<DatabaseServerReferenceMessage>,
    ) -> Result<Response<DatabaseServerMessage>, Status> {
        self.rocks_db_accessor.get_database_server(r)
    }

    /// Updates the configuration of an existing database server with the given data.
    async fn update_database_server(
        &self,
        r: Request<DatabaseServerMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        self.rocks_db_accessor.update_database_server(r)
    }

    /// Lists the domains associated with the specified database server.
    async fn list_domains(
        &self,
        r: Request<DatabaseServerReferenceMessage>,
    ) -> Result<Response<ArrayOfStringResponse>, Status> {
        self.rocks_db_accessor.list_domains(r)
    }

    /// Creates a new domain with the given configuration.
    async fn create_domain(
        &self,
        r: Request<DomainMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        self.rocks_db_accessor.create_domain(r)
    }

    /// Deletes an existing domain identified by the given reference.
    async fn delete_domain(
        &self,
        r: Request<DomainReferenceMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        self.rocks_db_accessor.delete_domain(r)
    }

    /// Retrieves the configuration of an existing domain identified by the given reference.
    async fn get_domain(
        &self,
        r: Request<DomainReferenceMessage>,
    ) -> Result<Response<DomainMessage>, Status> {
        self.rocks_db_accessor.get_domain(r)
    }

    /// Updates the configuration of an existing domain with the given data.
    async fn update_domain(
        &self,
        r: Request<DomainMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        self.rocks_db_accessor.update_domain(r)
    }

    /// Lists the tables associated with the specified domain.
    async fn list_tables(
        &self,
        r: Request<DomainReferenceMessage>,
    ) -> Result<Response<ArrayOfStringResponse>, Status> {
        self.rocks_db_accessor.list_tables(r)
    }

    /// Creates a new table with the given configuration.
    async fn create_table(
        &self,
        r: Request<TableMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        self.rocks_db_accessor.create_table(r)
    }

    /// Deletes an existing table identified by the given reference.
    async fn delete_table(
        &self,
        r: Request<TableReferenceMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        self.rocks_db_accessor.delete_table(r)
    }

    /// Retrieves the configuration of an existing table identified by the given reference.
    async fn get_table(
        &self,
        r: Request<TableReferenceMessage>,
    ) -> Result<Response<TableMessage>, Status> {
        self.rocks_db_accessor.get_table(r)
    }

    /// Updates the configuration of an existing table with the given data.
    async fn update_table(
        &self,
        r: Request<TableMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        self.rocks_db_accessor.update_table(r)
    }

    /// Lists the indexes associated with the specified table.
    async fn list_indexes(
        &self,
        r: Request<TableReferenceMessage>,
    ) -> Result<Response<ArrayOfStringResponse>, Status> {
        self.rocks_db_accessor.list_indexes(r)
    }

    /// Lists the values in the specified table.
    async fn list_values(
        &self,
        r: Request<TableReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status> {
        self.rocks_db_accessor.list_values(r)
    }

    /// Lists the values in the specified table with the given key prefix.
    async fn list_values_by_key_prefix(
        &self,
        r: Request<TableValueReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status> {
        self.rocks_db_accessor.list_values_by_key_prefix(r)
    }

    /// Lists the values in the specified table within the given ID range.
    async fn list_values_by_id_range(
        &self,
        r: Request<TableIdRangeReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status> {
        self.rocks_db_accessor.list_values_by_id_range(r)
    }

    /// Lists the values in the specified table with the given list of IDs.
    async fn list_values_by_id_list(
        &self,
        r: Request<TableIdListReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status> {
        self.rocks_db_accessor.list_values_by_id_list(r)
    }

    /// Creates a new index with the given configuration.
    async fn create_index(
        &self,
        r: Request<IndexMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        self.rocks_db_accessor.create_index(r)
    }

    /// Deletes an existing index identified by the given reference.
    async fn delete_index(
        &self,
        r: Request<IndexReferenceMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        self.rocks_db_accessor.delete_index(r)
    }

    /// Retrieves the configuration of an existing index identified by the given reference.
    async fn get_index(
        &self,
        r: Request<IndexReferenceMessage>,
    ) -> Result<Response<IndexMessage>, Status> {
        self.rocks_db_accessor.get_index(r)
    }

    /// Updates the configuration of an existing index with the given data.
    async fn update_index(
        &self,
        r: Request<IndexMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        self.rocks_db_accessor.update_index(r)
    }

    /// Creates a new value in the specified table with the given configuration.
    async fn create_value(
        &self,
        r: Request<CreateTableValueMessage>,
    ) -> Result<Response<OndoKeyMessage>, Status> {
        self.rocks_db_accessor.create_value(r)
    }

    /// Deletes an existing value identified by the given reference from the specified table.
    async fn delete_value(
        &self,
        r: Request<TableValueReferenceMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        self.rocks_db_accessor.delete_value(r)
    }

    /// Retrieves the value in the specified table identified by the given reference.
    async fn get_value(
        &self,
        r: Request<TableValueReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status> {
        self.rocks_db_accessor.get_value(r)
    }

    /// Updates an existing value in the specified table with the given data.
    async fn update_value(
        &self,
        r: Request<TableValueMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        self.rocks_db_accessor.update_value(r)
    }

    /// Finds values in the specified table based on the given indexed value reference.
    async fn find_values(
        &self,
        r: Request<IndexedValueReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status> {
        self.rocks_db_accessor.find_values(r)
    }

    /// Finds values in the specified table based on the given indexed value range reference.
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
