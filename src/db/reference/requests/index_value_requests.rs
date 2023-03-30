use crate::db::entity::IndexValue;
use crate::db::reference::IndexValueReference;
use crate::db::DbResult;
pub(crate) trait IndexValueRequests {
    fn get_index_value_stored(
        &self,
        cf_name: &str,
        key: &IndexValueReference,
    ) -> DbResult<Option<IndexValue>>;
}
