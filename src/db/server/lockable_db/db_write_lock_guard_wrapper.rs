use rocksdb::DB;
use std::sync::RwLockWriteGuard;
use tempfile::TempDir;

pub struct DbWriteLockGuardWrapper<'a> {
    pub guard: RwLockWriteGuard<'a, DB>,
    pub db_path: &'a String,
}

impl<'a> DbWriteLockGuardWrapper<'a> {
    pub(crate) fn new(tuple: (RwLockWriteGuard<'a, DB>, &'a Option<TempDir>, &'a String)) -> Self {
        Self {
            guard: tuple.0,
            db_path: tuple.2,
        }
    }
}

impl<'a> std::ops::Deref for DbWriteLockGuardWrapper<'a> {
    type Target = DB;

    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

impl<'a> std::ops::DerefMut for DbWriteLockGuardWrapper<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.guard
    }
}