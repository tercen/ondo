use crate::db::entity::domain::DomainStored;
use crate::db::entity::reference::cf_name::CfName;
use crate::db::entity::reference::domain_reference::DomainName;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DomainStoredEffect {
    Put(CfName, DomainName, DomainStored),
    Delete(CfName, DomainName),
}
