use crate::db::db_error::DbResult;
use crate::db::entity::index::IndexValue;
use crate::db::entity::reference::index_value_reference::IndexValueReference;
pub(crate) trait IndexValueRequests {
    fn get_index_value_stored(
        &self,
        cf_name: &str,
        key: &IndexValueReference,
    ) -> DbResult<Option<IndexValue>>;
}
