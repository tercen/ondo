use crate::db::entity::reference::database_server_reference::DatabaseServerName;
use crate::db::entity::database_server::DatabaseServerStored;
use crate::db::db_error::DbResult;
pub(crate) trait DatabaseServerStoredRequests {
    fn get_database_server_stored(
        &self,
        cf_name: &str,
        key: &DatabaseServerName,
    ) -> DbResult<Option<DatabaseServerStored>>;
}
