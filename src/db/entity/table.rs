//table.rs
use crate::db::entity::Index;
use crate::db::reference::TableReference;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub(crate) struct Table {
    pub reference: TableReference,
}

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub(crate) struct TableStored {
    pub table: Table,
    pub indexes: HashMap<String, Index>,
}
