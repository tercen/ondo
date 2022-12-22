use super::super::DataBaseServerStored;
use super::super::DataBaseServer;
use super::super::DbError;
use super::super::DbResult;
use super::CfName;

pub(super) trait DatabaseServerStoredReferenceTrait {
    type Effects;
    type Requests;

    fn cf_name(&self) -> String;
    fn get_database_server_stored(&self, requests: &Self::Requests) -> DbResult<DataBaseServerStored>;
    fn put_database_server_stored(db_server: DataBaseServerStored) -> DbResult<Self::Effects>;
    fn post_database_server_stored(db_server: DataBaseServerStored) -> DbResult<Self::Effects>;
    fn delete_database_server_stored(&self) -> DbResult<Self::Effects>;
    fn list_domain_names(&self, requests: &Self::Requests) -> DbResult<Vec<String>>;
}

pub trait DatabaseServerReferenceTrait {
    type Effects;
    type Requests;

    fn cf_name(&self) -> String;
    fn get_database_server(&self, requests: &Self::Requests) -> DbResult<DataBaseServer>;
    fn put_database_server(
        db_server_u: DataBaseServer,
        requests: &Self::Requests,
    ) -> DbResult<Self::Effects>;
    fn post_database_server(
        db_server_u: DataBaseServer,
        requests: &Self::Requests,
    ) -> DbResult<Self::Effects>;
    fn delete_database_server(&self) -> DbResult<Self::Effects>;
    fn list_domain_names(&self, requests: &Self::Requests) -> DbResult<Vec<String>>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatabaseServerReference;

impl DatabaseServerReference {
    pub fn new() -> Self {
        DatabaseServerReference
    }
}


