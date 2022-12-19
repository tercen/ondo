use super::super::DbError;
use super::super::Index;
use super::TableReference;

trait IndexReferenceTrait {
    type Effect; 
    type Request;

    fn get_index(&self, request: &Self::Request) -> Result<&Index, DbError>;
    fn put_index(index: Index) -> Self::Effect;
    fn remove_index(&self) -> Self::Effect;
    fn list_indexes(&self, request: &Self::Request) -> Result<Vec<Index>, DbError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexReference {
    pub index_name: String,
    pub table_name: String,
    pub domain_name: String,
}

impl IndexReference {
    pub fn new(domain_name: &str, table_name: &str, index_name: &str) -> Self {
        IndexReference {
            domain_name: domain_name.to_string(),
            table_name: table_name.to_string(),
            index_name: index_name.to_string(),
        }
    }

    pub fn to_table_reference(&self) -> TableReference {
        TableReference::new(&self.domain_name, &self.table_name)
    }
}