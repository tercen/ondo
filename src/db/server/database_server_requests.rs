use super::rocks_db_accessor::RocksDbAccessor;
use crate::db::server::database_server::stored::DatabaseServerStoredRequests;

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
        let db = self.db.get
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
