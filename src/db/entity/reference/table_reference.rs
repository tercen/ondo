use super::super::DbError;
use super::super::Table;

trait TableReferenceTrait {
    type Effect;
    type Requests;

    fn get_table(&self, requests: &Self::Requests) -> Result<Table, DbError>;
    fn put_table(table: Table) -> Self::Effect;
    fn post_table(table: Table) -> Self::Effect;
    fn delete_table(&self) -> Self::Effect;
    fn list_tables(&self, requests: &Self::Requests) -> Result<Vec<Table>, DbError>;
    fn cf_name(&self) -> String;
}

pub struct TableReference {
    pub table_name: String,
    pub domain_name: String,
}
