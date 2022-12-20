use super::super::DbError;
use super::super::Table;
use super::super::TableU;
use super::DomainReference;

pub trait TableReferenceTrait {
    type Effect;
    type Requests;

    fn cf_name(&self) -> String;
    fn get_table(&self, requests: &Self::Requests) -> Result<Table, DbError>;
    fn put_table(table: Table) -> Self::Effect;
    fn post_table(table: Table) -> Self::Effect;
    fn delete_table(&self) -> Self::Effect;
    fn list_tables(&self, requests: &Self::Requests) -> Result<Vec<Table>, DbError>;

    fn get_table_u(&self, requests: &Self::Requests) -> Result<TableU, DbError>;
    fn put_table_u(table_u: TableU, requests: &Self::Requests) -> Result<Self::Effect, DbError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableReference {
    pub table_name: String,
    pub domain_name: String,
}

impl TableReference {
    pub fn new(domain_name: &str, table_name: &str) -> Self {
        TableReference {
            domain_name: domain_name.to_string(),
            table_name: table_name.to_string(),
        }
    }

    pub fn to_domain_reference(&self) -> DomainReference {
        DomainReference::new(&self.domain_name)
    }
}