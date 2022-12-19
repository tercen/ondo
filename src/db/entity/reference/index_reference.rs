use super::super::DbError;
use super::super::Index;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexReference {
    pub index_name: String,
    pub table_name: String,
    pub domain_name: String,
}

trait IndexReferenceTrait {
    type Effect; 
    type Request;

    fn get_index(&self, request: &Self::Request) -> Result<&Index, DbError>;
    fn put_index(index: Index) -> Self::Effect;
    fn remove_index(&self) -> Self::Effect;
    fn list_indexes(&self, request: &Self::Request) -> Result<Vec<Index>, DbError>;
}