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
pub(crate) enum TransactionOrDbReadGuard<'a> {
    TransactionRead(
        PreferredTransactionReadLockGuardWrapper<'a>,
        PreferredDbReadLockGuardWrapper<'a>,
    ),
    DbRead(PreferredDbReadLockGuardWrapper<'a>),
}

impl<'a> TransactionOrDbReadGuard<'a> {
    pub(crate) fn inner<'b>(&'b self) -> TransactionOrDb<'b> {
        match self {
            TransactionOrDbReadGuard::TransactionRead(guard, db_guard) => {
                TransactionOrDb::Transaction(guard.deref(), db_guard.deref())
            }
            TransactionOrDbReadGuard::DbRead(guard) => TransactionOrDb::Db(guard.deref()),
        }
    }
    pub(crate) fn inner_older(&'a self) -> TransactionOrDb<'a> {
        match self {
            TransactionOrDbReadGuard::TransactionRead(guard, db_guard) => {
                TransactionOrDb::Transaction(guard.deref(), db_guard.deref())
            }
            TransactionOrDbReadGuard::DbRead(guard) => TransactionOrDb::Db(guard.deref()),
        }
    }

}

type PreferredTransactionWriteLockGuardWrapper<'a> =
    ReentrantMutexGuardWrapper<'a, Transaction<'a, TransactionDB>>;
type PreferredDbWriteLockGuardWrapper<'a> = DbWriteLockGuardWrapper<'a, TransactionDB>;

// #[derive(Debug)]
pub(crate) enum TransactionOrDbWriteGuard<'a> {
    TransactionWrite(
        PreferredTransactionWriteLockGuardWrapper<'a>,
        PreferredDbWriteLockGuardWrapper<'a>,
    ),
    DbWrite(PreferredDbWriteLockGuardWrapper<'a>),
}

impl<'a> TransactionOrDbWriteGuard<'a> {
    pub(crate) fn inner(&'a self) -> TransactionOrDb<'a> {
        match self {
            TransactionOrDbWriteGuard::TransactionWrite(guard, db_guard) => {
                TransactionOrDb::Transaction(guard.deref(), db_guard.deref())
            }
            TransactionOrDbWriteGuard::DbWrite(guard) => TransactionOrDb::Db(guard.deref()),
        }
    }
    pub(crate) fn inner_mut<'b>(&'b mut self) -> MutTransactionOrDb<'b> {
        match self {
            TransactionOrDbWriteGuard::TransactionWrite(guard, db_guard) => {
                MutTransactionOrDb::Transaction
            }

            TransactionOrDbWriteGuard::DbWrite(guard) => MutTransactionOrDb::Db(guard.deref_mut()),
        }
    }
}
