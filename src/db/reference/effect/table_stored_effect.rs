use crate::db::entity::TableStored;
use crate::db::reference::{CfName, TableName};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TableStoredEffect {
    Put(CfName, TableName, TableStored),
    Delete(CfName, TableName),
}
