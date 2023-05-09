use crate::db::reference::requests::IndexIteratorRequests;
use crate::db::server::lockable_db::transaction_or_db_guard::TransactionOrDbReadGuard;
use crate::db::server::lockable_db::transaction_maker::TransactionMaker;
use crate::db::DbResult;

pub(crate) enum IndexIteratorRequestsFactoryEnum<'a> {
    LockableDb(TransactionMaker<'a>),
    Mock, // todo!{}: Replace with the actual mock type when it's available
}

impl<'a> IndexIteratorRequestsFactoryEnum<'a> {
    pub(crate) fn new_lockable_db(lockable_db: &TransactionMaker<'a>) -> Self {
        let the_clone = lockable_db.clone();
        IndexIteratorRequestsFactoryEnum::LockableDb(the_clone)
    }

    pub(crate) fn new_mock() -> Self {
        IndexIteratorRequestsFactoryEnum::Mock
    }

    pub(crate) fn create_read_locked_requests(&'a self) -> DbResult<IndexIteratorRequestsEnum<'a>> {
        match self {
            IndexIteratorRequestsFactoryEnum::LockableDb(lockable_db) => {
                let db_wrapper = lockable_db.read();
                Ok(IndexIteratorRequestsEnum::DbWrapper(db_wrapper))
            }
            IndexIteratorRequestsFactoryEnum::Mock => {
                todo!()
            }
        }
    }
}

pub(crate) enum IndexIteratorRequestsEnum<'a> {
    DbWrapper(TransactionOrDbReadGuard<'a>),
    MockWrapper, // todo!: Replace with the actual mock type when it's available
}

impl<'a> IndexIteratorRequestsEnum<'a> {
    pub(crate) fn as_trait(&'a self) -> &'a dyn IndexIteratorRequests<'a> {
        match self {
            IndexIteratorRequestsEnum::DbWrapper(db_wrapper) => db_wrapper,
            IndexIteratorRequestsEnum::MockWrapper => {
                todo!()
            }
        }
    }
}
