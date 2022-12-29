use super::index::Index;
use super::reference::TableReference;
use std::collections::HashMap;

pub struct Table {
    pub id: TableReference,
}
pub struct TableStored {
    pub table: Table,
    pub indexes: HashMap<String, Index>,
}
