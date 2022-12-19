use super::super::DbError;
use super::super::DatabaseServer;

trait DatabaseServerReferenceTrait {
    type Effect;
    type Requests;

    fn get_db_server(&self, requests: &Self::Requests) -> Result<DatabaseServer, DbError>;
    fn put_db_server(db_server: DatabaseServer) -> Self::Effect;
    fn post_db_server(db_server: DatabaseServer) -> Self::Effect;
    fn delete_db_server(&self) -> Self::Effect;
    fn cf_name(&self) -> String;
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DatabaseServerReference;

