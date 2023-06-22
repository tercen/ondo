// transaction_or_db_guard.rs

use rocksdb::{Transaction, TransactionDB};
use std::ops::Deref;
use std::ops::DerefMut;

use super::{
    db_read_lock_guard_wrapper::DbReadLockGuardWrapper,
    db_write_lock_guard_wrapper::DbWriteLockGuardWrapper,
    reentrant_mutex_guard_wrapper::ReentrantMutexGuardWrapper,
    transaction_or_db::MutTransactionOrDb, transaction_or_db::TransactionOrDb,
};

type PreferredTransactionReadLockGuardWrapper<'a> =
    ReentrantMutexGuardWrapper<'a, Transaction<'a, TransactionDB>>;
type PreferredDbReadLockGuardWrapper<'a> = DbReadLockGuardWrapper<'a, TransactionDB>;

// #[derive(Debug)]
pub(crate) struct TransactionOrDbReadGuard<'a, 'b> {
    pub guard_pair: (
        PreferredDbReadLockGuardWrapper<'a>,
        Option<PreferredTransactionReadLockGuardWrapper<'b>>,
    ),
}

impl<'a, 'b> TransactionOrDbReadGuard<'a, 'b> {
    pub(crate) fn new(
        db: PreferredDbReadLockGuardWrapper<'a>,
        tr: Option<PreferredTransactionReadLockGuardWrapper<'b>>,
    ) -> Self {
        Self {
            guard_pair: (db, tr),
        }
    }

    pub(crate) fn inner<'c>(&'c self) -> TransactionOrDb<'c> {
        match &self.guard_pair {
            (db_guard, Some(transaction_guard)) => {
                TransactionOrDb::Transaction(transaction_guard.deref(), db_guard.deref())
            }
            (db_guard, None) => TransactionOrDb::Db(db_guard.deref()),
        }
    }
    pub(crate) fn inner_older(&'a self) -> TransactionOrDb<'a> {
        match &self.guard_pair {
            (db_guard, Some(transantion_guard)) => {
                TransactionOrDb::Transaction(transantion_guard.deref(), db_guard.deref())
            }
            (db_guard, None) => TransactionOrDb::Db(db_guard.deref()),
        }
    }
}

type PreferredTransactionWriteLockGuardWrapper<'a> =
    ReentrantMutexGuardWrapper<'a, Transaction<'a, TransactionDB>>;
type PreferredDbWriteLockGuardWrapper<'a> = DbWriteLockGuardWrapper<'a, TransactionDB>;

// #[derive(Debug)]
pub(crate) struct TransactionOrDbWriteGuard<'a, 'b> {
    guard_pair: (
        PreferredDbWriteLockGuardWrapper<'a>,
        Option<PreferredTransactionReadLockGuardWrapper<'b>>,
    ),
}

impl<'a, 'b> TransactionOrDbWriteGuard<'a, 'b> {
    pub(crate) fn new(
        db: PreferredDbWriteLockGuardWrapper<'a>,
        tr: Option<PreferredTransactionReadLockGuardWrapper<'b>>,
    ) -> Self {
        Self {
            guard_pair: (db, tr),
        }
    }

    pub(crate) fn inner(&'a self) -> TransactionOrDb<'a> {
        match &self.guard_pair {
            (db_guard, Some(transantion_guard)) => {
                TransactionOrDb::Transaction(transantion_guard.deref(), db_guard.deref())
            }
            (db_guard, None) => TransactionOrDb::Db(db_guard.deref()),
        }
    }
    pub(crate) fn inner_mut<'c>(&'c mut self) -> MutTransactionOrDb<'c> {
        match &self.guard_pair {
            (db_guard, Some(transantion_guard)) => MutTransactionOrDb::Transaction,
            (ref mut  db_guard, None) => MutTransactionOrDb::Db(db_guard.deref_mut()),
        }
    }
}
