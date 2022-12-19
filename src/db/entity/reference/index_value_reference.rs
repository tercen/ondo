use super::super::DbError;
use super::super::Key;
use super::super::IndexValue;

trait IdReferenceTrait {
    type Effect;
    type Requests;
    fn get_index_value(&self, requests: &Self::Requests) -> Result<IndexValue, DbError>;
    fn put_index_value(&self, id: IndexValue) -> Self::Effect;
    fn delete_index_value(&self) -> Self::Effect;
    fn list_index_values(&self, requests: &Self::Requests) -> Result<Vec<IndexValue>, DbError>;
    fn cf_name(&self) -> String;
}

struct IndexValueReference {
    pub key: Key,
    pub index_name: String,
    pub table_name: String,
    pub domain_name: String,
}

