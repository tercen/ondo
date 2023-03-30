use crate::db::db_error::{DbError, DbResult};
use crate::db::entity::DatabaseServerStored;
use crate::db::reference::DatabaseServerName;
use crate::db::reference::requests::DatabaseServerStoredRequests;
use crate::db::server::rocks_db_accessor::RocksDbAccessor;
use crate::db::server::source_sink::ondo_serializer::OndoSerializer;
use crate::db::DbError::CfNotFound;

impl DatabaseServerStoredRequests for RocksDbAccessor {
    fn get_database_server_stored(
        &self,
        cf_name: &str,
        key: &DatabaseServerName,
    ) -> DbResult<Option<DatabaseServerStored>> {
        let guarded_db = self.guarded_db();
        let db = RocksDbAccessor::db_read_lock(&guarded_db)?;
        let cf = db.cf_handle(cf_name).ok_or(CfNotFound)?;
        let ondo_key = DatabaseServerName::ondo_serialize(key)?;
        let answer = db
            .get_cf(cf, &ondo_key)
            .map_err(|err| DbError::RocksDbError(err))?;
        answer
            .map(|bytes| DatabaseServerStored::ondo_deserialize(&bytes))
            .transpose()
    }
}
