use crate::db::entity::DatabaseServerStored;
use crate::db::reference::DatabaseServerName;
use crate::db::DbResult;
pub(crate) trait DatabaseServerStoredRequests {
    fn get_database_server_stored(
        &self,
        cf_name: &str,
        key: &DatabaseServerName,
    ) -> DbResult<Option<DatabaseServerStored>>;
}
