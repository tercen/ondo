// transaction_or_db_guard.rs

use rocksdb::{Transaction, TransactionDB};
use std::ops::Deref;
use std::ops::DerefMut;

use super::mutex_guard_wrapper::MutexGuardWrapper;
use super::{
    db_read_lock_guard_wrapper::DbReadLockGuardWrapper,
    db_write_lock_guard_wrapper::DbWriteLockGuardWrapper, transaction_or_db::TransactionOrDb,
};

// #[derive(Debug)]
pub(crate) enum TransactionOrDbReadGuard<'a> {
    TransactionRead(MutexGuardWrapper<'a, Transaction<'a, TransactionDB>>),
    DbRead(DbReadLockGuardWrapper<'a, TransactionDB>),
}

impl<'a> Deref for TransactionOrDbReadGuard<'a> {
    type Target = TransactionOrDb<'a>;

    fn deref(&self) -> &Self::Target {
        match self {
            TransactionOrDbReadGuard::TransactionRead(guard) => {
                &TransactionOrDb::Transaction(guard.deref())
            }
            TransactionOrDbReadGuard::DbRead(guard) => &TransactionOrDb::Db(guard.deref()),
        }
    }
}

// #[derive(Debug)]
pub(crate) enum TransactionOrDbWriteGuard<'a> {
    TransactionWrite(MutexGuardWrapper<'a, Transaction<'a, TransactionDB>>),
    DbWrite(DbWriteLockGuardWrapper<'a, TransactionDB>),
}

impl<'a> Deref for TransactionOrDbWriteGuard<'a> {
    type Target = TransactionOrDb<'a>;

    fn deref(&self) -> &Self::Target {
        match self {
            TransactionOrDbWriteGuard::TransactionWrite(guard) => {
                &TransactionOrDb::Transaction(guard.deref())
            }
            TransactionOrDbWriteGuard::DbWrite(guard) => &TransactionOrDb::Db(guard.deref()),
        }
    }
}

impl<'a> DerefMut for TransactionOrDbWriteGuard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            TransactionOrDbWriteGuard::TransactionWrite(guard) => {
                &mut TransactionOrDb::Transaction(guard.deref_mut())
            }
            TransactionOrDbWriteGuard::DbWrite(guard) => {
                &mut TransactionOrDb::Db(guard.deref_mut())
            }
        }
    }
}
//FIXME: Implement get_for_update
