use crate::db::db_error::{DbError, DbResult};
use crate::db::entity::DomainStored;
use crate::db::reference::requests::DomainStoredRequests;
use crate::db::reference::DomainName;
use crate::db::server::lockable_db::LockableDb;
use crate::db::server::source_sink::ondo_serializer::OndoSerializer;
use crate::db::DbError::CfNotFound;

impl DomainStoredRequests for LockableDb {
    fn get_domain_stored(&self, cf_name: &str, key: &DomainName) -> DbResult<Option<DomainStored>> {
        let db = self.read();
        let cf = db.cf_handle(cf_name).ok_or(CfNotFound)?;
        let ondo_key = DomainName::ondo_serialize(key)?;
        let answer = db
            .get_cf(cf, &ondo_key)
            .map_err(|err| DbError::RocksDbError(err))?;
        answer
            .map(|bytes| DomainStored::ondo_deserialize(&bytes))
            .transpose()
    }
}
