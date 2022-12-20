use::std::collections::HashMap;
use super::reference::DomainReference;

pub struct DomainU {
    pub id: DomainReference
}
pub struct Domain {
    pub domain_u: DomainU,
    pub tables: HashMap<String, ()>
}
