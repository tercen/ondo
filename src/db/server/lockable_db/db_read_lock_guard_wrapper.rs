use rocksdb::DB;
use std::ops::Deref;
use std::sync::RwLockReadGuard;
use tempfile::TempDir;

pub struct DbReadLockGuardWrapper<'a> {
    pub guard: RwLockReadGuard<'a, DB>,
    pub db_path: &'a String,
}

impl<'a> DbReadLockGuardWrapper<'a> {
    pub(crate) fn new(tuple: (RwLockReadGuard<'a, DB>, &'a Option<TempDir>, &'a String)) -> Self {
        Self {
            guard: tuple.0,
            db_path: tuple.2,
        }
    }
}

impl<'a> Deref for DbReadLockGuardWrapper<'a> {
    type Target = DB;

    fn deref(&self) -> &Self::Target {
        &(self.guard)
    }
}
