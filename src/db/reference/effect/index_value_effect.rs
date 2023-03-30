use crate::db::entity::{IndexValue, OndoKey};
use crate::db::reference::CfName;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum IndexValueEffect {
    Put(CfName, OndoKey, IndexValue),
    Delete(CfName, OndoKey),
}
