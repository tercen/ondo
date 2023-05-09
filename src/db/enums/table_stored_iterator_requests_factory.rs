use crate::db::reference::requests::TableStoredIteratorRequests;
use crate::db::reference::table_reference::stored::MockTableStoredIteratorRequestsFactory;
use crate::db::reference::table_reference::stored::MockTableStoredIteratorTestRequests;
use crate::db::server::lockable_db::transaction_or_db_guard::TransactionOrDbReadGuard;
use crate::db::server::lockable_db::transaction_maker::TransactionMaker;
use crate::db::DbResult;

pub(crate) enum TableStoredIteratorRequestsFactoryEnum<'a> {
    LockableDb(TransactionMaker<'a>),
    Mock(MockTableStoredIteratorRequestsFactory),
}

impl<'a> TableStoredIteratorRequestsFactoryEnum<'a> {
    pub(crate) fn new_lockable_db(lockable_db: &TransactionMaker<'a>) -> Self {
        let the_clone = lockable_db.clone();
        TableStoredIteratorRequestsFactoryEnum::LockableDb(the_clone)
    }

    pub(crate) fn new_mock() -> Self {
        TableStoredIteratorRequestsFactoryEnum::Mock(MockTableStoredIteratorRequestsFactory {})
    }

    pub(crate) fn create_read_locked_requests(&'a self) -> DbResult<TableStoredIteratorRequestsEnum<'a>> {
        match self {
            TableStoredIteratorRequestsFactoryEnum::LockableDb(lockable_db) => {
                let db_wrapper = lockable_db.read();
                Ok(TableStoredIteratorRequestsEnum::DbWrapper(db_wrapper))
            }
            TableStoredIteratorRequestsFactoryEnum::Mock(_) => {
                let mock_requests = MockTableStoredIteratorTestRequests {};
                Ok(TableStoredIteratorRequestsEnum::MockWrapper(mock_requests))
            }
        }
    }
}

pub(crate) enum TableStoredIteratorRequestsEnum<'a> {
    DbWrapper(TransactionOrDbReadGuard<'a>),
    MockWrapper(MockTableStoredIteratorTestRequests),
}

impl<'a> TableStoredIteratorRequestsEnum<'a> {
    pub(crate) fn as_trait(&'a self) -> &'a dyn TableStoredIteratorRequests<'a> {
        match self {
            TableStoredIteratorRequestsEnum::DbWrapper(db_wrapper) => db_wrapper,
            TableStoredIteratorRequestsEnum::MockWrapper(mock_wrapper) => mock_wrapper,
        }
    }
}
