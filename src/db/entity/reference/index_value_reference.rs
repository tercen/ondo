use super::super::DbError;
use super::super::DbResult;
use super::super::Key;
use super::super::IndexValue;
use super::IndexReference;

pub trait IdReferenceTrait {
    type Effects;
    type Requests;
    fn cf_name(&self) -> String;
    fn get_index_value(&self, requests: &Self::Requests) -> DbResult<IndexValue>;
    fn put_index_value(&self, id: IndexValue) -> DbResult<Self::Effects>;
    fn delete_index_value(&self) -> DbResult<Self::Effects>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IndexValueReference {
    pub key: Key,
    pub index_name: String,
    pub table_name: String,
    pub domain_name: String,
}

impl IndexValueReference {
    pub fn new(domain_name: &str, table_name: &str, index_name: &str, key: Key) -> Self {
        IndexValueReference {
            domain_name: domain_name.to_string(),
            table_name: table_name.to_string(),
            index_name: index_name.to_string(),
            key,
        }
    }

    pub fn to_index_reference(&self) -> IndexReference {
        IndexReference::new(&self.domain_name, &self.table_name, &self.index_name)
    }
}   
