use super::DomainReference;
use super::super::DbError;
use super::super::DbResult;
use super::super::Table;
use super::super::TableStored;

pub trait TableStoredRequests {
    fn get_table_stored(&self, cf_name: &str, key: &TableReference) -> DbResult<Option<TableStored>>;    
}

pub enum TableStoredEffect {
    Put(String, TableReference, TableStored),
    Delete(String, TableReference),
}

pub(super) trait TableStoredReferenceTrait {
    type Effects;
    type Requests;

    fn cf_name(&self) -> String;
    fn get_table_stored(&self, requests: &Self::Requests) -> DbResult<Option<TableStored>>;
    fn put_table_stored(table_stored: TableStored) -> DbResult<Self::Effects>;
    fn post_table_stored(table_stored: TableStored) -> DbResult<Self::Effects>;
    fn delete_table_stored(&self) -> DbResult<Self::Effects>;
    fn list_index_names(&self, requests: &Self::Requests) -> DbResult<Vec<String>>;
}

pub trait TableReferenceTrait {
    type Effects;
    type Requests;

    fn get_table(&self, requests: &Self::Requests) -> DbResult<Option<Table>>;
    fn put_table(table: Table, requests: &Self::Requests) -> DbResult<Option<Self::Effects>>;
    fn post_table(table: Table, requests: &Self::Requests) -> DbResult<Self::Effects>;
    fn delete_table(&self) -> DbResult<Self::Effects>;
    fn list_index_names(&self, requests: &Self::Requests) -> DbResult<Vec<String>>;
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