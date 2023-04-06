use crate::db::reference::requests::TableStoredIteratorRequests;
use crate::db::reference::table_reference::stored::MockTableStoredIteratorRequestsFactory;
use crate::db::reference::table_reference::stored::MockTableStoredIteratorTestRequests;
use crate::db::server::rocks_db_accessor::DbArc;
use crate::db::server::rocks_db_accessor::DbReadLockGuardWrapper;
use crate::db::DbResult;

pub(crate) enum TableStoredIteratorRequestsFactoryEnum {
    DbArc(DbArc),
    Mock(MockTableStoredIteratorRequestsFactory),
}

impl TableStoredIteratorRequestsFactoryEnum {
    pub(crate) fn new_db_arc(db_arc: DbArc) -> Self {
        TableStoredIteratorRequestsFactoryEnum::DbArc(db_arc)
    }

    pub(crate) fn new_mock() -> Self {
        TableStoredIteratorRequestsFactoryEnum::Mock(MockTableStoredIteratorRequestsFactory {})
    }
    pub(crate) fn create_read_locked_requests<'a>(
        &'a self,
    ) -> DbResult<TableStoredIteratorRequestsEnum<'a>> {
        match self {
            TableStoredIteratorRequestsFactoryEnum::DbArc(db_arc) => {
                let db_wrapper = DbReadLockGuardWrapper::new(db_arc)?;
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
    DbWrapper(DbReadLockGuardWrapper<'a>),
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
