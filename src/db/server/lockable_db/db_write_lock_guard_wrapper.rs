use std::ops::{Deref, DerefMut};
use std::sync::RwLockWriteGuard;

pub struct DbWriteLockGuardWrapper<'a, T> {
    guard: RwLockWriteGuard<'a, T>,
    db_path: &'a String,
}

impl<'a, T> DbWriteLockGuardWrapper<'a, T> {
    pub(super) fn new(guard: RwLockWriteGuard<'a, T>, db_path: &'a String) -> Self {
        Self { guard, db_path }
    }
}

impl<'a, T> Deref for DbWriteLockGuardWrapper<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

impl<'a, T> DerefMut for DbWriteLockGuardWrapper<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.guard
    }
}
//TODO:XXX: Replace references to other guards with TransactionOrDbWriteGuard