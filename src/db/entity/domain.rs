use::std::collections::HashMap;
use super::reference::DomainReference;

pub struct Domain {
    pub id: DomainReference,
    pub tables: HashMap<String, ()>
}