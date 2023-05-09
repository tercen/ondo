// lockable_db/mod.rs
pub(crate) mod db_arc;
pub(crate) mod db_read_lock_guard_wrapper;
pub(crate) mod db_write_lock_guard_wrapper;
pub(crate) mod rocks_db_accessor;
pub(crate) mod transaction_maker;
pub(crate) mod transaction_or_db;
pub(crate) mod transaction_or_db_guard;
pub(crate) mod mutex_guard_wrapper;
pub mod version;

use db_arc::DbArc;
use db_read_lock_guard_wrapper::DbReadLockGuardWrapper;
use db_write_lock_guard_wrapper::DbWriteLockGuardWrapper;
use lazy_static::lazy_static;
use rocks_db_accessor::RocksDbAccessor;
use rocksdb::TransactionDB;
use version::Version;

lazy_static! {
    pub static ref LOCKABLE_DB: LockableDb = LockableDb::default();
}

#[derive(Clone)]
pub struct LockableDb { //FIXME: Should not be public
    db_arc: DbArc,
}

impl LockableDb {
    fn new(db_arc: DbArc) -> Self {
        Self { db_arc }
    }

    pub(super) fn default() -> Self {
        let accessor = RocksDbAccessor::default();
        Self::new(accessor.db_arc.clone())
    }

    pub(crate) fn in_memory() -> Self {
        let accessor = RocksDbAccessor::in_memory();
        Self::new(accessor.db_arc.clone())
    }

    pub(self) fn read(&self) -> DbReadLockGuardWrapper<'_, TransactionDB> {
        let guard = self.db_arc.db_lock.db.read().unwrap();
        let db_path = &self.db_arc.db_lock.db_path;
        DbReadLockGuardWrapper::new(guard, db_path)
    }

    pub(self) fn write(&self) -> DbWriteLockGuardWrapper<'_, TransactionDB> {
        let guard = self.db_arc.db_lock.db.write().unwrap();
        let db_path = &self.db_arc.db_lock.db_path;
        DbWriteLockGuardWrapper::new(guard, db_path)
    }

    pub fn db_path(&self) -> &str {
        &self.db_arc.db_lock.db_path
    }

    pub fn get_version(&self) -> Version {
        Version::new()
    }
}

//TODO:XXX: Replace other read()/write() subjects with transaction_maker 
