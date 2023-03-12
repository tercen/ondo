use crate::db::entity::database_server::DatabaseServerStored;
use crate::db::entity::reference::cf_name::CfName;
use crate::db::entity::reference::database_server_reference::DatabaseServerName;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DatabaseServerStoredEffect {
    Put(CfName, DatabaseServerName, DatabaseServerStored),
    Delete(CfName, DatabaseServerName),
}
