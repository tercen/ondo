use crate::db::entity::{OndoKey, TableValue};
use crate::db::reference::CfName;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TableValueEffect {
    Put(CfName, OndoKey, TableValue),
    Delete(CfName, OndoKey),
}
