use crate::db::entity::index::IndexValue;
use crate::db::entity::ondo_key::OndoKey;
use crate::db::entity::reference::cf_name::CfName;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum IndexValueEffect {
    Put(CfName, OndoKey, IndexValue),
    Delete(CfName, OndoKey),
}
