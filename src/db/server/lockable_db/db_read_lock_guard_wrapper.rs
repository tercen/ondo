use std::ops::Deref;
use std::sync::RwLockReadGuard;

pub struct DbReadLockGuardWrapper<'a, T> {
    guard: RwLockReadGuard<'a, T>,
    db_path: &'a String,
}

impl<'a, T> DbReadLockGuardWrapper<'a, T> {
    pub(super) fn new(guard: RwLockReadGuard<'a, T>, db_path: &'a String) -> Self {
        Self { guard, db_path }
    }
}

impl<'a, T> Deref for DbReadLockGuardWrapper<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}
//TODO:XXX: Replace references to other guards with TransactionOrDbReadGuard