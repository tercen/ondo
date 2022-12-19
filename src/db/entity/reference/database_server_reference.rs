use super::super::DbError;
use super::super::DatabaseServer;

pub trait DatabaseServerReferenceTrait {
    type Effect;
    type Requests;

    fn cf_name(&self) -> String;
    fn get_db_server(&self, requests: &Self::Requests) -> Result<DatabaseServer, DbError>;
    fn put_db_server(db_server: DatabaseServer) -> Self::Effect;
    fn post_db_server(db_server: DatabaseServer) -> Self::Effect;
    fn delete_db_server(&self) -> Self::Effect;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatabaseServerReference;

impl DatabaseServerReference {
    pub fn new() -> Self {
        DatabaseServerReference
    }
}

