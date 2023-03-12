use crate::db::entity::reference::cf_name::CfName;
use crate::db::entity::reference::table_reference::TableName;
use crate::db::entity::table::TableStored;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TableStoredEffect {
    Put(CfName, TableName, TableStored),
    Delete(CfName, TableName),
}
