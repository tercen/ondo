use crate::db::entity::{IndexValue, OndoKey};
use crate::db::DbResult;

pub(crate) trait IndexIteratorRequests<'a> {
    fn all_values_with_key_prefix(
        &'a self,
        value_cf_name: &str,
        key_prefix: OndoKey,
    ) -> DbResult<Box<dyn Iterator<Item = DbResult<IndexValue>> + 'a>>;
    fn all_values_with_key_range(
        &'a self,
        value_cf_name: &str,
        start_key_prefix: OndoKey,
        end_key_prefix: OndoKey,
    ) -> DbResult<Box<dyn Iterator<Item = DbResult<IndexValue>> + 'a>>;
}
