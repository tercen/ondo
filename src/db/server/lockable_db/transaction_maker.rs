//transaction_or_db_holder.rs
use super::db_read_lock_guard_wrapper::DbReadLockGuardWrapper;
use super::db_write_lock_guard_wrapper::DbWriteLockGuardWrapper;
use super::mutex_guard_wrapper::MutexGuardWrapper;
use super::transaction_or_db_guard::TransactionOrDbReadGuard;
use super::transaction_or_db_guard::TransactionOrDbWriteGuard;
use super::LOCKABLE_DB;
use super::version::Version;
use crate::db::db_error::DbError;
use rocksdb::{Transaction, TransactionDB};
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::Mutex;
use super::LockableDb;

#[derive(Clone)]
pub(crate) struct TransactionMaker<'a> {
    transaction: Option<Arc<Mutex<Transaction<'a, TransactionDB>>>>,
    lockable_db: LockableDb,
}

impl<'a> TransactionMaker<'a> {
    pub fn get_version(&self) -> Version {
        self.lockable_db.get_version()
    }
    pub fn new(lockable_db: LockableDb) -> Self {
        TransactionMaker {
            transaction: None,
            lockable_db,
        }
    }

    pub fn create_transaction(&mut self) {
        if self.transaction.is_none() {
            let lockable_db = self.lockable_db.read();
            let transaction = lockable_db.transaction();
            self.transaction = Some(Arc::new(Mutex::new(transaction)));
        }
    }

    pub fn commit_transaction(&mut self) -> Result<(), DbError> {
        if let Some(transaction) = self.transaction.take() {
            transaction
                .lock()
                .unwrap()
                .commit()
                .map_err(DbError::TransactionError)?;
        }
        Ok(())
    }

    pub fn abort_transaction(&mut self) -> Result<(), DbError> {
        if let Some(transaction) = self.transaction.take() {
            transaction
                .lock()
                .unwrap()
                .rollback()
                .map_err(DbError::TransactionError)?;
        }
        Ok(())
    }

    pub fn read(&self) -> TransactionOrDbReadGuard<'a> {
        if let Some(transaction) = &self.transaction {
            let guard = transaction.lock().unwrap();
            let db_path = self.lockable_db.db_path();
            TransactionOrDbReadGuard::TransactionRead(MutexGuardWrapper::new(guard, db_path))
        } else {
            let guard_wrapper = LOCKABLE_DB.read();
            TransactionOrDbReadGuard::DbRead(guard_wrapper)
        }
    }

    pub fn write(&self) -> TransactionOrDbWriteGuard<'a> {
        if let Some(transaction) = &self.transaction {
            let guard = transaction.lock().unwrap();
            let db_path = self.lockable_db.db_path();
            TransactionOrDbWriteGuard::TransactionWrite(MutexGuardWrapper::new(
                guard, &db_path.to_owned(),
            ))
        } else {
            let guard_wrapper = self.lockable_db.write();
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
    use crate::db::server::lockable_db::LockableDb;

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
        match *guard {
            TransactionOrDb::Db(_) => assert!(true),
            TransactionOrDb::Transaction(_) => assert!(false, "Expected Db, got Transaction"),
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
        match &mut *guard {
            //We are using a write lock to obtain a mutable reference to the guard
            TransactionOrDb::Transaction(_) => assert!(true),
            TransactionOrDb::Db(_) => assert!(false, "Expected Transaction, got Db"),
        }
    }
}

