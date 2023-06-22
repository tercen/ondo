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



//     pub fn commit_transaction(&mut self) -> Result<(), DbError> {
//         if let Some(transaction) = self.transaction.take() {
//             transaction.commit().map_err(DbError::TransactionError)?;
//         }
//         Ok(())
//     }

//     pub fn roll_back_transaction(&mut self) -> Result<(), DbError> {
//         if let Some(transaction) = self.transaction.take() {
//             transaction.rollback().map_err(DbError::TransactionError)?;
//         }
//         Ok(())
//     }


type LockableTransaction<'a> = Arc<ReentrantMutex<Transaction<'a, TransactionDB>>>;
#[derive(Clone)]
pub(crate) struct LockableTransactionOrDb<'a> {
    pub(crate) transaction: Option<LockableTransaction<'a>>,
    pub(crate) lockable_db: LockableDb,
}

impl<'a> LockableTransactionOrDb<'a> {
    fn make_transaction_lockable<'b>(tr: Transaction<'b, TransactionDB>)->LockableTransaction {
        Arc::new(ReentrantMutex::new(tr))
    }

    pub fn with_db(lockable_db: LockableDb) -> Self {
        Self { transaction: None, lockable_db }
    }

    pub fn with_transaction<'b>(lockable_db: LockableDb, transaction: Transaction<'b, TransactionDB>) -> Self {
        Self { transaction: Some(Self::make_transaction_lockable(transaction)), lockable_db }
    }

    pub fn get_version(&self) -> Version {
        self.lockable_db.get_version()
    }

    pub fn read<'b>(&'a self) -> TransactionOrDbReadGuard<'a, 'b> {
        let db_guard = self.lockable_db.read();
        match &self.transaction {
            None => TransactionOrDbReadGuard::new(db_guard, None),
            Some(transaction) => {
                let db_path = self.lockable_db.db_path();
                let transaction_guard = transaction.lock();
                let transaction_guard_wrapper =
                    ReentrantMutexGuardWrapper::new(transaction_guard, db_path.to_owned());
                TransactionOrDbReadGuard::new(db_guard, Some(transaction_guard_wrapper))
            }
        }
    }

    pub fn write<'b>(&'a self) -> TransactionOrDbWriteGuard<'a, 'b> { 
        let db_guard = self.lockable_db.write();
        match &self.transaction {
            None => TransactionOrDbWriteGuard::new(db_guard, None),
            Some(transaction) => {
                let db_path = self.lockable_db.db_path();
                let transaction_guard = transaction.lock();
                let transaction_guard_wrapper =
                    ReentrantMutexGuardWrapper::new(transaction_guard, db_path.to_owned());
                    TransactionOrDbWriteGuard::new(db_guard, Some(transaction_guard_wrapper))
            }
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
        db_read_lock_guard_wrapper::DbReadLockGuardWrapper, transaction_or_db::MutTransactionOrDb,
        LockableDb, LOCKABLE_DB,
    };

    #[test]
    fn test_read_returns_db() {
        // Arrange

        let lockable_transaction_or_db = LockableTransactionOrDb {
            transaction: None,
            lockable_db: LOCKABLE_DB.clone(),
        };

        // Act
        let guard = lockable_transaction_or_db.read();

        // Assert
        match guard.inner() {
            TransactionOrDb::Db(_) => assert!(true),
            TransactionOrDb::Transaction(_, _) => assert!(false, "Expected Db, got Transaction"),
        }
    }

    // #[test]
    // fn test_create_transaction_returns_transaction() {
    //     // Arrange
    //         // let lockable_db = LockableTransactionOrDb::with_db(LOCKABLE_DB.clone());
    //         let lockable_db = LockableTransactionOrDb::with_transaction(LockableDb::in_memory());

    //     // Act
    //     let lockable_db_or_transaction = transaction_maker.lockable_transaction();

    //     let mut guard = lockable_db_or_transaction.write();

    //     // Assert
    //     match &mut guard.inner_mut() {
    //         //We are using a write lock to obtain a mutable reference to the guard
    //         MutTransactionOrDb::Transaction => assert!(true),
    //         MutTransactionOrDb::Db(_) => assert!(false, "Expected Transaction, got Db"),
    //     }
    // }


}
