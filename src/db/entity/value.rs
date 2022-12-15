use super::DbError;
pub type Value = serde_json::Value;

struct ValueReference {
    pub id: Value,
    pub table_name: String,
    pub domain_name: String,
}
trait ValueReferenceTrait {
    type Effect;
    type Request;
    fn get_value(&self, request: &Self::Request) -> Result<Value, DbError>;
    fn put_value(&self, value: Value) -> Self::Effect;
    fn post_value(&self, value: Value) -> Self::Effect;
    fn delete_value(&self) -> Self::Effect;
    fn cf_name(&self) -> String;
}

