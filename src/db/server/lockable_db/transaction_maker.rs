use super::db_read_lock_guard_wrapper::DbReadLockGuardWrapper;
//transaction_or_db_holder.rs
use super::reentrant_mutex_guard_wrapper::ReentrantMutexGuardWrapper;
use super::transaction_or_db_guard::TransactionOrDbReadGuard;
use super::transaction_or_db_guard::TransactionOrDbWriteGuard;
use super::version::Version;
use super::LockableDb;
use crate::db::db_error::DbError;
use parking_lot::ReentrantMutex;
use rocksdb::{Transaction, TransactionDB};
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct TransactionMaker<'a> {
    transaction: Option<Arc<ReentrantMutex<Transaction<'a, TransactionDB>>>>,
    lockable_db: LockableDb,
    db_guard: Option<DbReadLockGuardWrapper<'a, TransactionDB>> //FIXME sometimes we have write lock
}

impl<'a> TransactionMaker<'a> {
    pub fn get_version(&self) -> Version {
        self.lockable_db.get_version()
    }
    pub fn new(lockable_db: LockableDb) -> Self {
        TransactionMaker {
            transaction: None,
            lockable_db,
            db_guard: None
        }
    }

    pub fn create_transaction(&mut self) {
        if self.transaction.is_none() {
            self.db_guard = Some(self.lockable_db.read());
            let transaction = self.guard.transaction();
            self.transaction = Some(Arc::new(ReentrantMutex::new(transaction)));
        }
    }

    pub fn commit_transaction(&mut self) -> Result<(), DbError> {
        if let Some(transaction) = self.transaction.take() {
            transaction
                .lock()
                .commit()
                .map_err(DbError::TransactionError)?;
        }
        Ok(())
    }

    pub fn abort_transaction(&mut self) -> Result<(), DbError> {
        if let Some(transaction) = self.transaction.take() {
            transaction
                .lock()
                .rollback()
                .map_err(DbError::TransactionError)?;
        }
        Ok(())
    }

    pub fn read(&self) -> TransactionOrDbReadGuard<'a> {
        let guard_wrapper = self.lockable_db.read();
        if let Some(transaction) = &self.transaction {
            let guard = transaction.lock();
            let db_path = self.lockable_db.db_path();
            TransactionOrDbReadGuard::TransactionRead(
                ReentrantMutexGuardWrapper::new(guard, db_path),
                guard_wrapper,
            )
        } else {
            TransactionOrDbReadGuard::DbRead(guard_wrapper)
        }
    }

    pub fn write(&self) -> TransactionOrDbWriteGuard<'a> {
        let guard_wrapper = self.lockable_db.write();
        if let Some(transaction) = &self.transaction {
            let guard = transaction.lock();
            let db_path = self.lockable_db.db_path();
            TransactionOrDbWriteGuard::TransactionWrite(
                ReentrantMutexGuardWrapper::new(guard, &db_path.to_owned()),
                guard_wrapper,
            )
        } else {
            TransactionOrDbWriteGuard::DbWrite(guard_wrapper)
        }
    }

    pub fn db_path(&self) -> &str {
        self.lockable_db.db_path()
    }
}

#[cfg(test)]
mod tests {
    use super::super::transaction_or_db::TransactionOrDb;
    use super::*;
    use crate::db::server::lockable_db::{
        transaction_or_db::MutTransactionOrDb, LockableDb, LOCKABLE_DB,
    };

    // Replace LOCKABLE_DB with LockableDb::in_memory() in the setup of tests
    fn setup() {
        *LOCKABLE_DB = LockableDb::in_memory();
    }

    #[test]
    fn test_read_returns_db() {
        // Arrange
        setup();
        let transaction_maker = TransactionMaker::new(LOCKABLE_DB.clone());

        // Act
        let guard = transaction_maker.read();

        // Assert
        match guard.inner() {
            TransactionOrDb::Db(_) => assert!(true),
            TransactionOrDb::Transaction(_, _) => assert!(false, "Expected Db, got Transaction"),
        }
    }

    #[test]
    fn test_create_transaction_returns_transaction() {
        // Arrange
        setup();
        let mut transaction_maker = TransactionMaker::new(LOCKABLE_DB.clone());

        // Act
        transaction_maker.create_transaction();
        let mut guard = transaction_maker.write();

        // Assert
        match &mut guard.inner_mut() {
            //We are using a write lock to obtain a mutable reference to the guard
            MutTransactionOrDb::Transaction => assert!(true),
            MutTransactionOrDb::Db(_) => assert!(false, "Expected Transaction, got Db"),
        }
    }
}
