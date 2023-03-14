// use rocksdb::{Options, DB};
use rocksdb::{Options, DB};
use std::sync::{Arc, RwLock};

// Define the struct that contains the RocksDB instance
#[derive(Clone)]
pub struct RocksDbAccessor {
    db: Arc<RwLock<DB>>,
    db_path: String,
    options: Options,
}

impl Default for RocksDbAccessor {
    fn default() -> Self {
        // Define the default path for the database
        let db_path = std::env::var("ONDO_DB_PATH").unwrap_or("./db/ondo_rocksdb".to_owned());

        // Create options for the RocksDB instance
        let mut options = Options::default();
        options.create_if_missing(true);
        // options.set_use_thread_local(true);

        let cf_names = DB::list_cf(&options, &db_path).unwrap_or(Vec::new());

        // Open the RocksDB instance at the default path
        let raw_db = DB::open_cf(&options, &db_path, cf_names).unwrap();

        // Wrap the RocksDB instance in an Arc and Mutex to ensure thread safety
        let db = Arc::new(RwLock::new(raw_db));

        // Return a new instance of MyStruct with the default RocksDB instance
        RocksDbAccessor {
            db,
            db_path,
            options,
        }
    }
}

impl RocksDbAccessor {
    pub fn guarded_db(&self) -> Arc<RwLock<DB>> {
        Arc::clone(&self.db)
    }
}
