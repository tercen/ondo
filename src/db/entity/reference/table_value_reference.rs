use super::super::DbError;
use super::super::Value;
trait ValueReferenceTrait {
    type Effect;
    type Request;
    fn get_value(&self, request: &Self::Request) -> Result<Value, DbError>;
    fn put_value(&self, value: Value) -> Self::Effect;
    fn post_value(&self, value: Value) -> Self::Effect;
    fn delete_value(&self) -> Self::Effect;
    fn list_values(&self, request: &Self::Request) -> Result<Vec<Value>, DbError>;
    fn cf_name(&self) -> String;
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValueReference {
    pub id: Value,
    pub table_name: String,
    pub domain_name: String,
}