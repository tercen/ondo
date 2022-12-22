use super::index::Index;
use super::reference::TableReference;
use std::collections::HashMap;

pub struct TableU {
    pub id: TableReference,
}
pub struct TableStored {
    pub table_u: TableU,
    pub indexes: HashMap<String, Index>,
}
