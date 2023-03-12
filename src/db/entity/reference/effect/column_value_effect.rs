use crate::db::entity::reference::cf_name::CfName;
use crate::db::entity::reference::column_value_reference::ColumnKey;
use crate::db::entity::reference::column_value_reference::ColumnValue;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ColumnValueEffect {
    Put(CfName, ColumnKey, ColumnValue),
    Delete(CfName, ColumnKey),
}
