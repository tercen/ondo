use super::reference::DomainReference;
use::std::collections::HashMap;

pub struct Domain {
    pub id: DomainReference
}
pub struct DomainStored {
    pub domain: Domain,
    pub tables: HashMap<String, ()>
}
