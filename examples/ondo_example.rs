//$ rm -R db/ondo_rocksdb/; cargo run --example ondo_example

use ondo::db::server::{
    database_server_trait::DatabaseServerTrait, domain_server_trait::DomainServerTrait,
    rocks_db_accessor::RocksDbAccessor, table_server_trait::TableServerTrait,
};
use ondo::ondo_remote::*;
use tonic::Request;

fn main() {
    let rda = RocksDbAccessor::default();
    database_server_example(&rda);
}

fn database_server_example(rda: &RocksDbAccessor) {
    let database_server_reference_msg = DatabaseServerReferenceMessage {};
    let database_server_msg = DatabaseServerMessage {};
    let version = rda.version(Request::new(EmptyMessage {}));
    println!("Version: {:?}", version);
    let answer = rda.create_database_server(Request::new(database_server_msg.clone()));
    println!("Created Database: {:?}", answer);
    let answer = rda.get_database_server(Request::new(database_server_reference_msg.clone()));
    println!("Got Database: {:?}", answer);
    let answer = rda.update_database_server(Request::new(database_server_msg));
    println!("Updated Database: {:?}", answer);
    let answer = rda.list_domains(Request::new(DatabaseServerReferenceMessage {}));
    println!("Listed Domains: {:?}", answer);
    domain_server_example(rda);
    let answer = rda.delete_database_server(Request::new(DatabaseServerReferenceMessage {}));
    println!("Deleted Database: {:?}", answer);
}

fn domain_server_example(rda: &RocksDbAccessor) {
    let domain_name = "test_domain";
    let domain_reference_msg = DomainReferenceMessage {
        domain_name: domain_name.to_owned(),
    };
    let domain_msg = DomainMessage {
        domain_reference: Some(domain_reference_msg.clone()),
    };
    let answer = rda.create_domain(Request::new(domain_msg.clone()));
    println!("Created Domain: {:?}", answer);
    let answer = rda.get_domain(Request::new(domain_reference_msg.clone()));
    println!("Got Domain: {:?}", answer);
    let answer = rda.update_domain(Request::new(domain_msg.clone()));
    println!("Updated Domain: {:?}", answer);
    let answer = rda.list_tables(Request::new(domain_reference_msg.clone()));
    println!("Listed Tables: {:?}", answer);
    table_server_example(rda, &domain_reference_msg);
    let answer = rda.delete_domain(Request::new(domain_reference_msg.clone()));
    println!("Deleted Domain: {:?}", answer);
}

fn table_server_example(rda: &RocksDbAccessor, domain_reference_msg: &DomainReferenceMessage) {
    let table_name = "test_table";
    let table_reference_msg = TableReferenceMessage {
        domain_reference: Some(domain_reference_msg.clone()),
        table_name: table_name.to_owned(),
    };
    let table_msg = TableMessage {
        table_reference: Some(table_reference_msg.clone()),
    };
    let answer = rda.create_table(Request::new(table_msg.clone()));
    println!("Created Table: {:?}", answer);
    let answer = rda.get_table(Request::new(table_reference_msg.clone()));
    println!("Got Table: {:?}", answer);
    let answer = rda.update_table(Request::new(table_msg.clone()));
    println!("Updated Table: {:?}", answer);
    let answer = rda.list_indexes(Request::new(table_reference_msg.clone()));
    println!("Listed Tables: {:?}", answer);
    let answer = rda.delete_table(Request::new(table_reference_msg.clone()));
    println!("Deleted Table: {:?}", answer);
    println!("TODO list functions not yet implemented")
}
