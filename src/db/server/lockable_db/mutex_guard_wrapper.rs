use std::ops::{Deref, DerefMut};
use std::sync::MutexGuard;

pub struct MutexGuardWrapper<'a, T> {
    guard: MutexGuard<'a, T>,
    db_path: &'a str,
}

impl<'a, T> MutexGuardWrapper<'a, T> {
    pub(super) fn new(guard: MutexGuard<'a, T>, db_path: &'a str) -> Self {
        Self { guard, db_path }
    }
}

impl<'a, T> Deref for MutexGuardWrapper<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

impl<'a, T> DerefMut for MutexGuardWrapper<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.guard
    }
}
