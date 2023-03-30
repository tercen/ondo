use crate::db::reference::{CfName, ColumnKey, ColumnValue};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ColumnValueEffect {
    Put(CfName, ColumnKey, ColumnValue),
    Delete(CfName, ColumnKey),
}
