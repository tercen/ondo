use super::super::DbError;
use super::super::DbResult;
use super::super::DomainStored;
use super::super::Domain;
use super::DatabaseServerReference;

trait DomainStoredReferenceTrait {
    type Effects;
    type Requests;

    fn cf_name(&self) -> String;
    fn get_domain_stored(&self, requests: &Self::Requests) -> DbResult<DomainStored>;
    fn put_domain_stored(domain: DomainStored) -> DbResult<Self::Effects>;
    fn post_domain_stored(domain: DomainStored) -> DbResult<Self::Effects>;
    fn delete_domain_stored(&self) -> DbResult<Self::Effects>;
    fn list_table_names(&self, requests: &Self::Requests) -> DbResult<Vec<String>>;
}

trait DomainReferenceTrait {
    type Effects;
    type Requests;

    fn cf_name(&self) -> String;
    fn get_domain(&self, requests: &Self::Requests) -> DbResult<Domain>;
    fn put_domain(domain_u: Domain, requests: &Self::Requests) ->DbResult<Self::Effects>;
    fn post_domain(domain_u: Domain, requests: &Self::Requests)
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
