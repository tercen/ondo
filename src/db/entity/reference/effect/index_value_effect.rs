use crate::db::entity::index::IndexValue;
use crate::db::entity::reference::cf_name::CfName;
use crate::db::entity::reference::index_value_reference::IndexValueReference;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum IndexValueEffect {
    Put(CfName, IndexValueReference, IndexValue), //FIXME: Use Column Value instead of IndexValueEffect
    Delete(CfName, IndexValueReference),
}
