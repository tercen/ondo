use super::CfName;
use super::super::DatabaseServer;
use super::super::DatabaseServerStored;
use super::super::DbError;
use super::super::DbResult;

pub trait DatabaseServerStoredRequests {
    fn get_database_server_stored(&self, cf_name: &str, key: &DatabaseServerReference) -> DbResult<Option<DatabaseServerStored>>;    
}

pub enum DatabaseServerStoredEffect {
    Put(String, DatabaseServerReference, DatabaseServerStored),
    Delete(String, DatabaseServerReference),
}

pub type Effects = Vec<DatabaseServerStoredEffect>;
pub trait Requests: DatabaseServerStoredRequests {}

pub(super) trait DatabaseServerStoredReferenceTrait {
    fn cf_name(&self) -> String;
    fn get_database_server_stored(&self, requests: &dyn Requests) -> DbResult<DatabaseServerStored>;
    fn put_database_server_stored(data_base_server: DatabaseServerStored) -> DbResult<Effects>;
    fn post_database_server_stored(data_base_server: DatabaseServerStored) -> DbResult<Effects>;
    fn delete_database_server_stored(&self) -> DbResult<Effects>;
    fn list_domain_names(&self, requests: &dyn Requests) -> DbResult<Vec<String>>;
}

pub trait DatabaseServerReferenceTrait {
    type Effects;
    type Requests;

    fn get_database_server(&self, requests: &dyn Requests) -> DbResult<DatabaseServer>;
    fn put_database_server(
        data_base_server: DatabaseServer,
        requests: &dyn Requests,
    ) -> DbResult<Effects>;
    fn post_database_server(
        db_server_u: DatabaseServer,
        requests: &dyn Requests,
    ) -> DbResult<Effects>;
    fn delete_database_server(&self) -> DbResult<Effects>;
    fn list_domain_names(&self, requests: &dyn Requests) -> DbResult<Vec<String>>;
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatabaseServerReference;

impl DatabaseServerReference {
    pub fn new() -> Self {
        DatabaseServerReference
    }
}

impl DatabaseServerStoredReferenceTrait for DatabaseServerReference {

    fn cf_name(&self) -> String {
        CfName::for_server_meta()
    }

    fn get_database_server_stored(&self, requests: &dyn Requests) -> DbResult<DatabaseServerStored> {
        todo!()
    }

    fn put_database_server_stored(data_base_server: DatabaseServerStored) -> DbResult<Effects> {
        todo!()
    }

    fn post_database_server_stored(data_base_server: DatabaseServerStored) -> DbResult<Effects> {
        todo!()
    }

    fn delete_database_server_stored(&self) -> DbResult<Effects> {
        todo!()
    }

    fn list_domain_names(&self, requests: &dyn Requests) -> DbResult<Vec<String>> {
        todo!()
    }
}


impl DatabaseServerReferenceTrait for DatabaseServerReference {
    type Effects = ();
    type Requests = ();

    fn get_database_server(&self, requests: &dyn Requests) -> DbResult<DatabaseServer> {
        todo!()
    }

    fn put_database_server(
        data_base_server: DatabaseServer,
        requests: &dyn Requests,
    ) -> DbResult<Effects> {
        todo!()
    }

    fn post_database_server(
        data_base_server: DatabaseServer,
        requests: &dyn Requests,
    ) -> DbResult<Effects> {
        todo!()
    }

    fn delete_database_server(&self) -> DbResult<Effects> {
        todo!()
    }

    fn list_domain_names(&self, requests: &dyn Requests) -> DbResult<Vec<String>> {
        todo!()
    }
}
