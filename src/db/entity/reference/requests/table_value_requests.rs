use crate::db::db_error::DbResult;
use crate::db::entity::reference::table_value_reference::TableValueReference;
use crate::db::entity::table_value::TableValue;

pub(crate) trait TableValueRequests {
    //FIXME: Use Column Value instead of TableValueRequests
    fn get_table_value(
        &self,
        cf_name: &str,
        key: &TableValueReference,
    ) -> DbResult<Option<TableValue>>;
}
