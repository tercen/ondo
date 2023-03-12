//table.rs
use std::collections::HashMap;
use super::index::Index;
use super::reference::TableReference;

#[derive(PartialEq, Eq, Clone, Debug)]
pub(crate) struct Table {
    pub id: TableReference,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub(crate) struct TableStored {
    pub table: Table,
    pub indexes: HashMap<String, Index>,
}
