use super::DbError;
use super::index::Key;

type Id = super::index::Value;
struct IdReference {
    pub key: Key,
    pub index_name: String,
    pub table_name: String,
    pub domain_name: String,
}

trait IdReferenceTrait {
    type Effect;
    type Requests;
    fn get_id(&self, requests: &Self::Requests) -> Result<Id, DbError>;
    fn put_id(&self, id: Id) -> Self::Effect;
    fn delete_id(&self) -> Self::Effect;
    fn cf_name(&self) -> String;
}

