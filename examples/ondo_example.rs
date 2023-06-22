//$ rm -R db/ondo_rocksdb/; cargo run --example ondo_example
use ondo::db::server::{
    database_server_trait::DatabaseServerTrait,
    domain_server_trait::DomainServerTrait,
    lockable_db::LOCKABLE_DB,
    table_server_trait::TableServerTrait,
    table_value_server_trait::TableValueServerTrait,
};
use ondo::ondo_remote::*;
use serde::{Deserialize, Serialize};
use tonic::Request;

fn main() {
    let rda = LockableTransactionOrDb::with_db(LOCKABLE_DB.clone());
    database_server_example(&rda);
}

fn database_server_example(rda: &LockableTransactionOrDb) {
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

fn domain_server_example(rda: &LockableTransactionOrDb) {
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

fn table_server_example(rda: &LockableTransactionOrDb, domain_reference_msg: &DomainReferenceMessage) {
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
    table_value_server_example(rda, &table_reference_msg);
    let answer = rda.delete_table(Request::new(table_reference_msg.clone()));
    println!("Deleted Table: {:?}", answer);
    println!("TODO list functions not yet implemented")
}

fn table_value_server_example(
    rda: &LockableTransactionOrDb,
    table_reference_msg: &TableReferenceMessage,
) {
    println!("!!! Table Value Server Example !!!");

    #[derive(Serialize, Deserialize)]
    struct Person {
        pub name: String,
        pub age: u32,
    }

    let mut person = Person {
        age: 42,
        name: "Bob".to_owned(),
    };

    let no_key = OptionalOndoKeyMessage {
        ondo_key: None,
    };
    let create_table_value_reference_msg = CreateTableValueReferenceMessage {
        table_reference: Some(table_reference_msg.clone()),
        key: Some(no_key),
    };
    let create_table_value_msg = CreateTableValueMessage {
        create_table_value_reference: Some(create_table_value_reference_msg),
        json: serde_json::to_string(&person).unwrap(),
    };

    let answer = rda.create_value(Request::new(create_table_value_msg.clone()));
    println!("Created Value: {:?}", answer);
    let new_ondo_key = answer.unwrap().get_ref().clone();
    let table_value_reference_msg = TableValueReferenceMessage {
        table_reference: Some(table_reference_msg.clone()),
        key: Some(new_ondo_key), //Update reference with new id
    };

    let answer = rda.get_value(Request::new(table_value_reference_msg.clone()));
    println!("Got Value: {:?}", answer);

    person.age = 43;
    let table_value_msg2 = TableValueMessage {
        table_value_reference: Some(table_value_reference_msg.clone()),
        json: serde_json::to_string(&person).unwrap(),
    };

    let answer = rda.update_value(Request::new(table_value_msg2.clone()));
    println!("Updated Value: {:?}", answer);
    let answer = rda.delete_value(Request::new(table_value_reference_msg.clone()));
    println!("Deleted Value: {:?}", answer);
}
