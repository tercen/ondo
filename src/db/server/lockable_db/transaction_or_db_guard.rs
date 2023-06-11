// transaction_or_db_guard.rs

use rocksdb::{Transaction, TransactionDB};
use std::ops::Deref;
use std::ops::DerefMut;

use super::{
    db_read_lock_guard_wrapper::DbReadLockGuardWrapper,
    db_write_lock_guard_wrapper::DbWriteLockGuardWrapper,
    reentrant_mutex_guard_wrapper::ReentrantMutexGuardWrapper, transaction_or_db::TransactionOrDb,
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

impl<'a> Deref for TransactionOrDbReadGuard<'a> {
    type Target = TransactionOrDb<'a>;

    fn deref(&self) -> &Self::Target {
        match self {
            TransactionOrDbReadGuard::TransactionRead(guard, db_guard) => {
                &TransactionOrDb::Transaction(guard.deref(), db_guard.deref())
            }
            TransactionOrDbReadGuard::DbRead(guard) => &TransactionOrDb::Db(guard.deref()),
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

impl<'a> Deref for TransactionOrDbWriteGuard<'a> {
    type Target = TransactionOrDb<'a>;

    fn deref(&self) -> &Self::Target {
        match self {
            TransactionOrDbWriteGuard::TransactionWrite(guard, db_guard) => {
                &TransactionOrDb::Transaction(guard.deref(), db_guard.deref())
            }
            TransactionOrDbWriteGuard::DbWrite(guard) => &TransactionOrDb::Db(guard.deref()),
        }
    }
}

impl<'a> DerefMut for TransactionOrDbWriteGuard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            TransactionOrDbWriteGuard::TransactionWrite(guard, db_guard) => {
                &mut TransactionOrDb::Transaction(guard.deref_mut(), db_guard.deref_mut())
            }
            TransactionOrDbWriteGuard::DbWrite(guard) => {
                &mut TransactionOrDb::Db(guard.deref_mut())
            }
        }
    }
}
//FIXME: Implement get_for_update
