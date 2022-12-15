use::std::collections::HashMap;
use super::DbError;
pub struct DomainReference {
    pub domain_name: String
}

trait DomainReferenceTrait {
    type Effect;
    type Requests;

    fn get_domain(&self, requests: &Self::Requests) -> Result<Domain, DbError>;
    fn put_domain(domain: Domain) -> Self::Effect;
    fn post_domain(domain: Domain) -> Self::Effect;
    fn delete_domain(&self) -> Self::Effect;
}

pub struct Domain {
    pub id: DomainReference,
    pub tables: HashMap<String, ()>,
}