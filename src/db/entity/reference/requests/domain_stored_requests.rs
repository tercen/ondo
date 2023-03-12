use crate::db::entity::reference::domain_reference::DomainName;
use crate::db::entity::domain::DomainStored;
use crate::db::db_error::DbResult;
pub(crate) trait DomainStoredRequests {
    fn get_domain_stored(&self, cf_name: &str, key: &DomainName) -> DbResult<Option<DomainStored>>;
}
