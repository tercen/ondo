use crate::db::db_error::DbError;
use crate::db::db_error::DbResult;
use rocksdb::{Options, DB};
use std::sync::{Arc, RwLock, RwLockReadGuard};
use tempfile::TempDir;

type DbArc = Arc<(RwLock<DB>, Option<TempDir>)>;

#[derive(Clone)]
pub struct RocksDbAccessor {
    db: DbArc,
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
        let db_path = std::env::var("ONDO_DB_PATH").unwrap_or("./db/ondo_rocksdb".to_owned());

        let mut options = Options::default();
        options.create_if_missing(true);

        Self::init(db_path, options)
    }
}

impl RocksDbAccessor {
    fn init(db_path: String, options: Options) -> Self {
        let cf_names = DB::list_cf(&options, &db_path).unwrap_or(Vec::new());
        let raw_db = DB::open_cf(&options, &db_path, cf_names).unwrap();
        let db = Arc::new((RwLock::new(raw_db), None));

        RocksDbAccessor {
            db,
            db_path,
            options,
        }
    }

    pub fn in_memory() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temporary directory");
        let db_path = temp_dir.path().to_string_lossy().into_owned();

        let mut options = Options::default();
        options.create_if_missing(true);

        Self::init(db_path, options).with_temp_dir(temp_dir)
    }

    pub fn with_temp_dir(mut self, temp_dir: TempDir) -> Self {
        let mut db = Arc::get_mut(&mut self.db).unwrap();
        db.1 = Some(temp_dir);
        self
    }
    
    pub fn guarded_db(&self) -> DbArc {
        Arc::clone(&self.db)
    }

    pub fn db_read_lock(guarded_db: &DbArc) -> DbResult<std::sync::RwLockReadGuard<DB>> {
        guarded_db.0.read().map_err(|_| DbError::CanNotLockDbMutex)
    }

    pub fn db_write_lock(guarded_db: &DbArc) -> DbResult<std::sync::RwLockWriteGuard<DB>> {
        guarded_db.0.write().map_err(|_| DbError::CanNotLockDbMutex)
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

impl<'a> DbReadLockGuardWrapper<'a> {
    pub(crate) fn new(
        guarded_db: &'a DbArc,
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

        assert!(RocksDbAccessor::db_read_lock(&db_accessor.db).is_ok());
        assert!(db_accessor.db.0.read().is_ok());
        assert!(db_accessor.db.as_ref().1.is_some());
    }
}
