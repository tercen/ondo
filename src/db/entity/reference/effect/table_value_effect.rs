use crate::db::entity::ondo_key::OndoKey;
use crate::db::entity::reference::cf_name::CfName;
use crate::db::entity::table_value::TableValue;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TableValueEffect {
    Put(CfName, OndoKey, TableValue),
    Delete(CfName, OndoKey),
}
