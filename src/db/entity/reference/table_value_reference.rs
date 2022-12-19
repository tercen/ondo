use super::super::DbError;
use super::super::Value;
use super::TableReference;

pub trait TableValueReferenceTrait {
    type Effect;
    type Request;
    fn cf_name(&self) -> String;
    fn get_value(&self, request: &Self::Request) -> Result<Value, DbError>;
    fn put_value(&self, value: Value) -> Self::Effect;
    fn post_value(&self, value: Value) -> Self::Effect;
    fn delete_value(&self) -> Self::Effect;
    fn list_values(&self, request: &Self::Request) -> Result<Vec<Value>, DbError>;
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableValueReference {
    pub id: Value,
    pub table_name: String,
    pub domain_name: String,
}

impl TableValueReference {
    pub fn new(domain_name: &str, table_name: &str, id: Value) -> Self {
        TableValueReference {
            domain_name: domain_name.to_string(),
            table_name: table_name.to_string(),
            id,
        }
    }

    pub fn to_table_reference(&self) -> TableReference {
        TableReference::new(&self.domain_name, &self.table_name)
    }
}