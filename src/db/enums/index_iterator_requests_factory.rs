use crate::db::server::lockable_db::transaction_or_db::TransactionOrDb;
use crate::db::server::lockable_db::transaction_or_db_guard::TransactionOrDbReadGuard;
use crate::db::server::lockable_db::transaction_maker::LockableTransactionOrDb;
use crate::db::DbResult;

pub(crate) enum IndexIteratorRequestsFactoryEnum<'a> {
    LockableDb(LockableTransactionOrDb<'a>),
    Mock, // todo!{}: Replace with the actual mock type when it's available
}

impl<'a> IndexIteratorRequestsFactoryEnum<'a> {
    pub(crate) fn new_lockable_db(lockable_db: &LockableTransactionOrDb<'a>) -> Self {
        let the_clone = lockable_db.clone();
        IndexIteratorRequestsFactoryEnum::LockableDb(the_clone)
    }

    // pub(crate) fn new_mock() -> Self {
    //     IndexIteratorRequestsFactoryEnum::Mock
    // }

    pub(crate) fn guard<'b>(&'b self) -> DbResult<IndexIteratorRequestsGuard<'a>> {
        match self {
            IndexIteratorRequestsFactoryEnum::LockableDb(lockable_db) => {
                Ok(IndexIteratorRequestsGuard::DbWrapper(lockable_db.read()))
            }
            IndexIteratorRequestsFactoryEnum::Mock => {
                todo!()
            }
        }
    }
}

pub(crate) enum IndexIteratorRequestsGuard<'a> {
    DbWrapper(TransactionOrDbReadGuard<'a>),
    MockWrapper, // todo!: Replace with the actual mock type when it's available
}

impl<'a> IndexIteratorRequestsGuard<'a> {
    pub(crate) fn get_transaction_or_db<'b>(&'b self) -> TransactionOrDb<'b> {
        match self {
            IndexIteratorRequestsGuard::DbWrapper(guard) => {
                let db = guard.inner();
                db
            },
            IndexIteratorRequestsGuard::MockWrapper => {
                todo!()
            }
        }
    }
}
