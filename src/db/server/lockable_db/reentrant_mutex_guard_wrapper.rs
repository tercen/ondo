use parking_lot::ReentrantMutexGuard;
use std::ops::Deref;

pub struct ReentrantMutexGuardWrapper<'a, T> {
    guard: ReentrantMutexGuard<'a, T>,
    db_path: &'a str,
}

impl<'a, T> ReentrantMutexGuardWrapper<'a, T> {
    pub(super) fn new(guard: ReentrantMutexGuard<'a, T>, db_path: &'a str) -> Self {
        Self { guard, db_path }
    }
}

impl<'a, T> Deref for ReentrantMutexGuardWrapper<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}
