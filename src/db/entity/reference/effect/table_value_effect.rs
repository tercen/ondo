use crate::db::entity::reference::cf_name::CfName;
use crate::db::entity::table_value::TableValue;
use crate::db::entity::ondo_key::OndoKey;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TableValueEffect {
    Put(CfName, OndoKey, TableValue), //FIXME: Use Column Value instead of TableValueEffect
    Delete(CfName, OndoKey),
}
