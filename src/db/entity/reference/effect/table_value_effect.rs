use crate::db::entity::reference::cf_name::CfName;
use crate::db::entity::reference::table_value_reference::TableValueReference;
use crate::db::entity::table_value::TableValue;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TableValueEffect {
    Put(CfName, TableValueReference, TableValue), //FIXME: Use Column Value instead of TableValueEffect
    Delete(CfName, TableValueReference),
}
