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

pub(crate) struct TransactionMaker<'a> {
    transaction: Option<Transaction<'a, TransactionDB>>,
    lockable_db: LockableDb,
}

impl<'a> TransactionMaker<'a> {
    pub fn new(lockable_db: LockableDb) -> Self {
        TransactionMaker {
            transaction: None,
            lockable_db,
        }
    }

    pub fn lockable_transaction(&mut self) -> LockableTransactionOrDb<'a> {
        match self.transaction {
            None => {
                let guard = self.lockable_db.read();
                let transaction = guard.transaction();
                self.transaction = Some(transaction);
                LockableTransactionOrDb {
                    transaction: Some(Arc::new(ReentrantMutex::new(transaction))),
                    lockable_db: self.lockable_db,
                }
                }
            Some(transaction) => 
                LockableTransactionOrDb {
                    transaction: Some(Arc::new(ReentrantMutex::new(transaction))),
                    lockable_db: self.lockable_db,
                }
            
        }
    }

    pub fn lockable_db(&self) -> LockableTransactionOrDb<'a> {
        LockableTransactionOrDb {
            transaction: None,
            lockable_db: self.lockable_db,
        }

    }

    pub fn commit_transaction(&mut self) -> Result<(), DbError> {
        if let Some(transaction) = self.transaction.take() {
            transaction.commit().map_err(DbError::TransactionError)?;
        }
        Ok(())
    }

    pub fn abort_transaction(&mut self) -> Result<(), DbError> {
        if let Some(transaction) = self.transaction.take() {
            transaction
                .rollback()
                .map_err(DbError::TransactionError)?;
        }
        Ok(())
    }
}

/// LockableTransactionOrDb can only be created by TransactionMaker
#[derive(Clone)]
pub(crate) struct LockableTransactionOrDb<'a> {
    transaction: Option<Arc<ReentrantMutex<Transaction<'a, TransactionDB>>>>,
    lockable_db: LockableDb,
}

impl<'a> LockableTransactionOrDb<'a> {
    pub fn get_version(&self) -> Version {
        self.lockable_db.get_version()
    }
   

    pub fn read(&'a self) -> TransactionOrDbReadGuard<'a> {
        let guard_wrapper = self.lockable_db.read();
        if let Some(transaction) = &self.transaction {
            let guard = transaction.lock();
            let db_path = self.lockable_db.db_path();
            TransactionOrDbReadGuard::TransactionRead(
                ReentrantMutexGuardWrapper::new(guard, db_path.to_owned()),
                guard_wrapper,
            )
        } else {
            TransactionOrDbReadGuard::DbRead(guard_wrapper)
        }
    }

    pub fn write(&'a self) -> TransactionOrDbWriteGuard<'a> {
        let guard_wrapper = self.lockable_db.write();
        if let Some(transaction) = &self.transaction {
            let guard = transaction.lock();
            let db_path = self.lockable_db.db_path();
            TransactionOrDbWriteGuard::TransactionWrite(
                ReentrantMutexGuardWrapper::new(guard, db_path.to_owned()),
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
        let lockable_db_or_transaction = transaction_maker.lockable_db();

        // Act
        let guard = lockable_db_or_transaction.read();

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
        let lockable_db_or_transaction = transaction_maker.lockable_transaction();

        let mut guard = lockable_db_or_transaction.write();

        // Assert
        match &mut guard.inner_mut() {
            //We are using a write lock to obtain a mutable reference to the guard
            MutTransactionOrDb::Transaction => assert!(true),
            MutTransactionOrDb::Db(_) => assert!(false, "Expected Transaction, got Db"),
        }
    }

    #[test]
    fn test_transaction_lifecycle() {
        setup();
        let mut transaction_maker = TransactionMaker::new(LOCKABLE_DB.clone());
        let lockable_db_or_transaction = transaction_maker.lockable_transaction();
        let result = transaction_maker.commit_transaction();
        assert!(result.is_ok());
    }
}
