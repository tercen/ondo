use crate::db::db_error::DbResult;
use crate::db::entity::reference::column_value_reference::ColumnKey;
use crate::db::entity::reference::column_value_reference::ColumnValue;

pub(crate) trait ColumnValueRequests {
    fn get_column_value(&self, cf_name: &str, key: &ColumnKey) -> DbResult<Option<ColumnValue>>;
}
