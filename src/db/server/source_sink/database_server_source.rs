use crate::db::db_error::DbResult;
use crate::db::entity::database_server::DatabaseServerStored;
use crate::db::entity::reference::database_server_reference::DatabaseServerName;
use crate::db::entity::reference::requests::database_server_stored_requests::DatabaseServerStoredRequests;
use crate::db::server::rocks_db_accessor::RocksDbAccessor;

// pub trait DatabaseServerStoredRequests {
//     fn get_database_server_stored(
//         &self,
//         cf_name: &str,
//         key: &DatabaseServerName,
//     ) -> DbResult<Option<DatabaseServerStored>>;
// }

impl DatabaseServerStoredRequests for RocksDbAccessor {
    fn get_database_server_stored(
        &self,
        cf_name: &str,
        _: &DatabaseServerName,
    ) -> DbResult<Option<DatabaseServerStored>> {
        todo!()
    }
}

// use bincode::{deserialize, serialize};
// use std::io::{Cursor, Result};

// #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
// struct MyStruct {
//     a: i32,
//     b: String,
// }

// fn main() -> Result<()> {
//     let my_struct = MyStruct {
//         a: 42,
//         b: "hello world".to_owned(),
//     };

//     // Convert struct to binary
//     let bytes = serialize(&my_struct)?;
//     println!("Bytes: {:?}", bytes);

//     // Convert binary to struct
//     let mut cursor = Cursor::new(bytes);
//     let decoded: MyStruct = deserialize_from(&mut cursor)?;
//     println!("Decoded: {:?}", decoded);

//     Ok(())
// }
