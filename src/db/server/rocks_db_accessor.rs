// use rocksdb::{Options, DB};
use crate::db::db_error::DbError;
use crate::db::db_error::DbResult;
use rocksdb::{Options, DB};
use std::sync::{Arc, RwLock, RwLockReadGuard};
use tempfile::TempDir;

// Define the struct that contains the RocksDB instance
#[derive(Clone)]
pub struct RocksDbAccessor {
    db: Arc<RwLock<DB>>,
    db_path: String,
    options: Options,
    temp_dir: Option<Arc<TempDir>>, // We have to keep temp_dir in scope, so that it is alive as long as the database is alive
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
        let db_path = std::env::var("ONDO_DB_PATH").unwrap_or("./db/ondo_rocksdb".to_owned());

        let mut options = Options::default();
        options.create_if_missing(true);

        Self::init(db_path, options, None)
    }
}

impl RocksDbAccessor {
    fn init(db_path: String, options: Options, temp_dir: Option<Arc<TempDir>>) -> Self {
        let cf_names = DB::list_cf(&options, &db_path).unwrap_or(Vec::new());
        let raw_db = DB::open_cf(&options, &db_path, cf_names).unwrap();
        let db = Arc::new(RwLock::new(raw_db));

        RocksDbAccessor {
            db,
            db_path,
            options,
            temp_dir,
        }
    }

    pub fn in_memory() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temporary directory");
        let db_path = temp_dir.path().to_string_lossy().into_owned();

        let mut options = Options::default();
        options.create_if_missing(true);

        Self::init(db_path, options, Some(Arc::new(temp_dir)))
    }

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

// In the same module where DbReadLockGuardWrapper is defined

impl<'a> DbReadLockGuardWrapper<'a> {
    pub(crate) fn new(
        guarded_db: &'a Arc<RwLock<DB>>,
    ) -> Result<DbReadLockGuardWrapper<'a>, DbError> {
        let guard = RocksDbAccessor::db_read_lock(guarded_db)?;
        Ok(DbReadLockGuardWrapper { guard })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_memory() {
        let db_accessor = RocksDbAccessor::in_memory();

        assert!(db_accessor.db.read().is_ok());
        assert!(db_accessor.temp_dir.is_some());
    }
}
