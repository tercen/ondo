//table.rs
use super::index::Index;
use super::reference::TableReference;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub(crate) struct Table {
    pub id: TableReference,
}

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub(crate) struct TableStored {
    pub table: Table,
    pub indexes: HashMap<String, Index>,
}
