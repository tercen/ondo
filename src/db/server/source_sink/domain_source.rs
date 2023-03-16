use crate::db::db_error::DbError::CfNotFound;
use crate::db::db_error::{DbError, DbResult};
use crate::db::entity::domain::DomainStored;
use crate::db::entity::reference::domain_reference::DomainName;
use crate::db::entity::reference::requests::domain_stored_requests::DomainStoredRequests;
use crate::db::server::rocks_db_accessor::RocksDbAccessor;
use crate::db::server::source_sink::ondo_serializer::OndoSerializer;

impl DomainStoredRequests for RocksDbAccessor {
    fn get_domain_stored(&self, cf_name: &str, key: &DomainName) -> DbResult<Option<DomainStored>> {
        let guarded_db = self.guarded_db();
        let db = RocksDbAccessor::db_read_lock(&guarded_db)?;
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
