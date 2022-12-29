use std::collections::HashMap;
use super::index::Index;
use super::reference::TableReference;

pub struct Table {
    pub id: TableReference,
}
pub struct TableStored {
    pub table: Table,
    pub indexes: HashMap<String, Index>,
}
