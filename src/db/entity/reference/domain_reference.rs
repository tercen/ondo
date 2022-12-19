use super::super::DbError;
use super::super::Domain;
use super::DatabaseServerReference;

trait DomainReferenceTrait {
    type Effect;
    type Requests;

    fn get_domain(&self, requests: &Self::Requests) -> Result<Domain, DbError>;
    fn put_domain(domain: Domain) -> Self::Effect;
    fn post_domain(domain: Domain) -> Self::Effect;
    fn delete_domain(&self) -> Self::Effect;
    fn list_domains(&self, requests: &Self::Requests) -> Result<Vec<Domain>, DbError>;
    fn cf_name(&self) -> String;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainReference {
    pub domain_name: String
}

impl DomainReference {
    pub fn new(domain_name: &str) -> Self {
        DomainReference {
            domain_name: domain_name.to_string()
        }
    }

    pub fn to_database_server_reference(&self) -> DatabaseServerReference {
        DatabaseServerReference::new()
    }
} 
