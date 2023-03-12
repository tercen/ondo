use rocksdb::{Options, DB};
use std::sync::{Arc, Mutex, MutexGuard};

use crate::db::db_error::DbError;

// Define the struct that contains the RocksDB instance
#[derive(Clone)]
pub struct RocksDbAccessor {
    _db: Arc<Mutex<DB>>,
}

impl Default for RocksDbAccessor {
    fn default() -> Self {
        // Define the default path for the database
        let db_path = "./db/ondo_rocksdb";

        // Create options for the RocksDB instance
        let mut options = Options::default();
        options.create_if_missing(true);
        // options.set_use_thread_local(true);

        // Open the RocksDB instance at the default path
        let raw_db = DB::open(&options, &db_path).unwrap();

        // Wrap the RocksDB instance in an Arc and Mutex to ensure thread safety
        let _db = Arc::new(Mutex::new(raw_db));

        // Return a new instance of MyStruct with the default RocksDB instance
        RocksDbAccessor { _db }
    }
}

impl RocksDbAccessor {
    pub fn db_guard(&self) -> Result<MutexGuard<DB>, DbError> {
        self._db.lock().map_err(|_| DbError::CanNotLockDbMutex)
    }
}
