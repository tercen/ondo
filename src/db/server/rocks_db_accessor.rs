// use rocksdb::{Options, DB};
use crate::db::db_error::DbError;
use crate::db::db_error::DbResult;
use rocksdb::{Options, DB};
use std::sync::{Arc, RwLock, RwLockReadGuard};

// Define the struct that contains the RocksDB instance
#[derive(Clone)]
pub struct RocksDbAccessor {
    db: Arc<RwLock<DB>>,
    db_path: String,
    options: Options,
}

pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub commit: String,
    pub date: String,
    pub features: String,
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

    pub fn db_read_lock(guarded_db: &Arc<RwLock<DB>>) -> DbResult<std::sync::RwLockReadGuard<DB>> {
        guarded_db.read().map_err(|_| DbError::CanNotLockDbMutex)
    }

    pub fn db_write_lock(
        guarded_db: &Arc<RwLock<DB>>,
    ) -> DbResult<std::sync::RwLockWriteGuard<DB>> {
        guarded_db.write().map_err(|_| DbError::CanNotLockDbMutex)
    }

    pub fn get_version(&self) -> Version {
        let ver = match semver::Version::parse(option_env!("VERSION").unwrap_or("0.0.0")) {
            Ok(ver) => ver,
            Err(_) => semver::Version::parse("0.0.0").unwrap(),
        };

        Version {
            major: ver.major,
            minor: ver.minor,
            patch: ver.patch,
            commit: option_env!("COMMIT_NUMBER")
                .map(|env| env.to_string())
                .unwrap_or("".to_string()),
            date: option_env!("BUILD_DATE")
                .map(|env| env.to_string())
                .unwrap_or("".to_string()),
            features: "".to_string(),
        }
    }
}

pub(crate) struct DbReadLockGuardWrapper<'a> {
    pub(crate) guard: RwLockReadGuard<'a, DB>,
}
