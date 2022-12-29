use::std::collections::HashMap;
use super::reference::DomainReference;

pub struct Domain {
    pub id: DomainReference
}
pub struct DomainStored {
    pub domain: Domain,
    pub tables: HashMap<String, ()>
}
