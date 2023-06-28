use crate::db::entity::TableValue;
use crate::db::reference::TableValueReference;
use crate::db::DbResult;

pub(crate) trait TableValueRequests {
    fn get_table_value(
        &self,
        cf_name: &str,
        key: &TableValueReference,
    ) -> DbResult<Option<TableValue>>;
    fn get_table_value_for_update(
        &self,
        cf_name: &str,
        key: &TableValueReference,
    ) -> DbResult<Option<TableValue>>;
}
