use super::DatabaseServerReference;
use super::super::DbError;
use super::super::DbResult;
use super::super::Domain;
use super::super::DomainStored;

pub trait DomainStoredRequests {
    fn get_domain_stored(&self, cf_name: &str, key: &DomainReference) -> DbResult<Option<DomainStored>>;    
}

pub enum DomainStoredEffect {
    Put(String, DomainReference, DomainStored),
    Delete(String, DomainReference),
}

pub(super) trait DomainStoredReferenceTrait {
    type Effects;
    type Requests;

    fn cf_name(&self) -> String;
    fn get_domain_stored(&self, requests: &Self::Requests) -> DbResult<DomainStored>;
    fn put_domain_stored(domain_stored: DomainStored) -> DbResult<Self::Effects>;
    fn post_domain_stored(domain_stored: DomainStored) -> DbResult<Self::Effects>;
    fn delete_domain_stored(&self) -> DbResult<Self::Effects>;
    fn list_table_names(&self, requests: &Self::Requests) -> DbResult<Vec<String>>;
}

pub trait DomainReferenceTrait {
    type Effects;
    type Requests;

    fn get_domain(&self, requests: &Self::Requests) -> DbResult<Domain>;
    fn put_domain(domain: Domain, requests: &Self::Requests) ->DbResult<Self::Effects>;
    fn post_domain(domain: Domain, requests: &Self::Requests)
        -> DbResult<Self::Effects>;
    fn delete_domain(&self) -> DbResult<Self::Effects>;
    fn list_table_names(&self, requests: &Self::Requests) -> DbResult<Vec<String>>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainReference {
    pub domain_name: String,
}

impl DomainReference {
    pub fn new(domain_name: &str) -> Self {
        DomainReference {
            domain_name: domain_name.to_string(),
        }
    }

    pub fn to_database_server_reference(&self) -> DatabaseServerReference {
        DatabaseServerReference::new()
    }
}
