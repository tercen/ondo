use super::super::DatabaseServer;
use super::super::DatabaseServerU;
use super::super::DbError;

pub trait DatabaseServerReferenceTrait {
    type Effect;
    type Requests;

    fn cf_name(&self) -> String;
    fn get_database_server(&self, requests: &Self::Requests) -> Result<DatabaseServer, DbError>;
    fn put_database_server(db_server: DatabaseServer) -> Self::Effect;
    fn post_database_server(db_server: DatabaseServer) -> Self::Effect;
    fn delete_db_server(&self) -> Self::Effect;

    fn get_database_server_u(&self, requests: &Self::Requests) -> Result<DatabaseServerU, DbError>;
    fn put_database_server_u(
        db_server_u: DatabaseServerU,
        requests: &Self::Requests,
    ) -> Result<Self::Effect, DbError>;

    fn list_domains(&self, requests: &Self::Requests) -> Result<Vec<String>, DbError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatabaseServerReference;

impl DatabaseServerReference {
    pub fn new() -> Self {
        DatabaseServerReference
    }
}
