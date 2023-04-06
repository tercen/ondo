use crate::db::reference::requests::IndexIteratorRequests;
use crate::db::server::rocks_db_accessor::DbArc;
use crate::db::server::rocks_db_accessor::DbReadLockGuardWrapper;
use crate::db::DbResult;

pub(crate) enum IndexIteratorRequestsFactoryEnum {
    DbArc(DbArc),
    Mock, // todo!{}: Replace with the actual mock type when it's available
}

impl IndexIteratorRequestsFactoryEnum {
    pub(crate) fn new_db_arc(db_arc: DbArc) -> Self {
        IndexIteratorRequestsFactoryEnum::DbArc(db_arc)
    }

    pub(crate) fn new_mock() -> Self {
        IndexIteratorRequestsFactoryEnum::Mock
    }
    pub(crate) fn create_read_locked_requests<'a>(
        &'a self,
    ) -> DbResult<IndexIteratorRequestsEnum<'a>> {
        match self {
            IndexIteratorRequestsFactoryEnum::DbArc(db_arc) => {
                let db_wrapper = DbReadLockGuardWrapper::new(db_arc)?;
                Ok(IndexIteratorRequestsEnum::DbWrapper(db_wrapper))
            }
            IndexIteratorRequestsFactoryEnum::Mock => {
                todo!()
            }
        }
    }
}

pub(crate) enum IndexIteratorRequestsEnum<'a> {
    DbWrapper(DbReadLockGuardWrapper<'a>),
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
