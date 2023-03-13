//domain.rs
use super::reference::DomainReference;
use ::std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub(crate) struct Domain {
    pub id: DomainReference,
}

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub(crate) struct DomainStored {
    pub domain: Domain,
    pub tables: HashMap<String, ()>,
}
