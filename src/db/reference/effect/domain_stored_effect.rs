use crate::db::entity::DomainStored;
use crate::db::reference::{CfName, DomainName};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DomainStoredEffect {
    Put(CfName, DomainName, DomainStored),
    Delete(CfName, DomainName),
}
