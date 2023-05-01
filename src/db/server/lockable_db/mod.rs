// lockable_db/mod.rs
pub(crate) mod db_read_lock_guard_wrapper;
pub(crate) mod db_write_lock_guard_wrapper;
mod rocks_db_accessor;
pub mod version;

use db_read_lock_guard_wrapper::DbReadLockGuardWrapper;
use db_write_lock_guard_wrapper::DbWriteLockGuardWrapper;
use lazy_static::lazy_static;
use rocks_db_accessor::DbArc;
use rocks_db_accessor::RocksDbAccessor;
use version::Version;

lazy_static! {
    pub static ref LOCKABLE_DB: LockableDb = LockableDb::default();
}

#[derive(Clone, Debug)]
pub struct LockableDb {
    db_arc: DbArc,
}

impl LockableDb {
    fn new(db_arc: DbArc) -> Self {
        Self { db_arc }
    }

    pub fn default() -> Self {
        let accessor = RocksDbAccessor::default();
        Self::new(accessor.db_arc.clone())
    }

    pub fn in_memory() -> Self {
        let accessor = RocksDbAccessor::in_memory();
        Self::new(accessor.db_arc.clone())
    }

    pub fn read(&self) -> DbReadLockGuardWrapper {
        let rw_lock = &self.db_arc.0; // Access the RwLock inside the Arc
        let guard = rw_lock.read().unwrap(); // Call the read method on the RwLock
        let temp_dir = &self.db_arc.1;
        let db_path = &self.db_arc.2;
        DbReadLockGuardWrapper::new((guard, temp_dir, db_path)) // Construct the wrapper
    }

    pub fn write(&self) -> DbWriteLockGuardWrapper {
        let rw_lock = &self.db_arc.0; // Access the RwLock inside the Arc
        let guard = rw_lock.write().unwrap(); // Call the write method on the RwLock
        let temp_dir = &self.db_arc.1;
        let db_path = &self.db_arc.2;
        DbWriteLockGuardWrapper::new((guard, temp_dir, db_path)) // Construct the wrapper
    }

    pub fn db_path(&self) -> &str {
        let db_path = &self.db_arc.2;
        db_path
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

impl Default for LockableDb {
    fn default() -> Self {
        Self::default()
    }
}
