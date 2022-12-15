use super::index::Index;
use super::DbError;
use std::collections::HashMap;

pub struct TableReference {
    pub table_name: String,
    pub domain_name: String,
}
trait TableReferenceTrait {
    type Effect;
    type Requests;

    fn get_table(&self, requests: &Self::Requests) -> Result<Table, DbError>;
    fn put_table(table: Table) -> Self::Effect;
    fn post_table(table: Table) -> Self::Effect;
    fn delete_table(&self) -> Self::Effect;
}

struct Table {
    pub id: TableReference,
    pub indexes: HashMap<String, Index>,
}