use crate::db::entity::DatabaseServerStored;
use crate::db::reference::{CfName, DatabaseServerName};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DatabaseServerStoredEffect {
    Put(CfName, DatabaseServerName, DatabaseServerStored),
    Delete(CfName, DatabaseServerName),
}
