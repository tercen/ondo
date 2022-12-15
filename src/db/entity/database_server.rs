use super::DbError;
use std::collections::HashMap;

struct DbServerReference;

trait DbServerReferenceTrait {
    type Effect;
    type Requests;

    fn get_db_server(&self, requests: &Self::Requests) -> Result<DbServer, DbError>;
    fn put_db_server(db_server: DbServer) -> Self::Effect;
    fn post_db_server(db_server: DbServer) -> Self::Effect;
    fn delete_db_server(&self) -> Self::Effect;
}

struct DbServer {
    pub domains: HashMap<String, ()>,
}