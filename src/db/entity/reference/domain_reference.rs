use super::super::DbError;
use super::super::Domain;
use super::super::DomainU;
use super::DatabaseServerReference;

trait DomainReferenceTrait {
    type Effect;
    type Requests;

    fn cf_name(&self) -> String;
    fn get_domain(&self, requests: &Self::Requests) -> Result<Domain, DbError>;
    fn put_domain(domain: Domain) -> Self::Effect;
    fn post_domain(domain: Domain) -> Self::Effect;
    fn delete_domain(&self) -> Self::Effect;

    fn get_domain_u(&self, requests: &Self::Requests) -> Result<DomainU, DbError>;
    fn put_domain_u(domain_u: DomainU, requests: &Self::Requests) -> Result<Self::Effect, DbError>;
    fn post_domain_u(domain_u: DomainU, requests: &Self::Requests)
        -> Result<Self::Effect, DbError>;

    fn list_tables(&self, requests: &Self::Requests) -> Result<Vec<String>, DbError>;
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
