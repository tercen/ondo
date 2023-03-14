//$ rm -R db/ondo_rocksdb/; cargo run --example ondo_example

use ondo::db::server::{
    database_server_trait::DatabaseServerTrait, rocks_db_accessor::RocksDbAccessor,
};
use ondo::ondo_remote::*;
use tonic::Request;

fn main() {
    let rda = RocksDbAccessor::default();
    let version = rda.version(Request::new(EmptyMessage {}));
    println!("Version: {:?}", version);
    let answer = rda.create_database_server(Request::new(DatabaseServerMessage {}));
    println!("Created Database: {:?}", answer);
    let answer = rda.get_database_server(Request::new(DatabaseServerReferenceMessage {}));
    println!("Got Database: {:?}", answer);
    let answer = rda.delete_database_server(Request::new(DatabaseServerReferenceMessage {}));
    println!("Deleted Database: {:?}", answer);
}
