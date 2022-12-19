use super::index::Index;
use super::reference::TableReference;
use std::collections::HashMap;

pub struct Table {
    pub id: TableReference,
    pub indexes: HashMap<String, Index>,
}
