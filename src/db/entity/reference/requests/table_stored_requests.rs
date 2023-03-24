use crate::db::db_error::DbResult;
use crate::db::entity::reference::table_reference::TableName;
use crate::db::entity::table::TableStored;
use crate::db::entity::table_value::TableValue;

pub(crate) trait TableStoredRequests {
    fn get_table_stored(&self, cf_name: &str, key: &TableName) -> DbResult<Option<TableStored>>;
    fn all_values(&self, value_cf_name: &str) -> Box<dyn Iterator<Item = TableValue>>;
}
