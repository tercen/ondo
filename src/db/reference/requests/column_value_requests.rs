use crate::db::reference::{ColumnKey, ColumnValue};
use crate::db::DbResult;

pub(crate) trait ColumnValueRequests {
    fn get_column_value(&self, cf_name: &str, key: &ColumnKey) -> DbResult<Option<ColumnValue>>;
}
