use crate::db::entity::DomainStored;
use crate::db::reference::DomainName;
use crate::db::DbResult;
pub(crate) trait DomainStoredRequests {
    fn get_domain_stored(&self, cf_name: &str, key: &DomainName) -> DbResult<Option<DomainStored>>;
}
