use super::CfName;
use super::super::DataBaseServer;
use super::super::DataBaseServerStored;
use super::super::DbError;
use super::super::DbResult;

pub trait DatabaseServerStoredRequests {
    fn get_database_server_stored(&self, cf_name: &str, key: &DatabaseServerReference) -> DbResult<Option<DataBaseServerStored>>;    
}

pub enum DatabaseServerStoredEffect {
    Put(DataBaseServerStored),
    Post(DataBaseServerStored),
    Delete,
}
pub(super) trait DatabaseServerStoredReferenceTrait {
    type Effects;
    type Requests;

    fn cf_name(&self) -> String;
    fn get_database_server_stored(&self, requests: &Self::Requests) -> DbResult<DataBaseServerStored>;
    fn put_database_server_stored(data_base_server: DataBaseServerStored) -> DbResult<Self::Effects>;
    fn post_database_server_stored(data_base_server: DataBaseServerStored) -> DbResult<Self::Effects>;
    fn delete_database_server_stored(&self) -> DbResult<Self::Effects>;
    fn list_domain_names(&self, requests: &Self::Requests) -> DbResult<Vec<String>>;
}

pub trait DatabaseServerReferenceTrait {
    type Effects;
    type Requests;

    fn cf_name(&self) -> String;
    fn get_database_server(&self, requests: &Self::Requests) -> DbResult<DataBaseServer>;
    fn put_database_server(
        data_base_server: DataBaseServer,
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

impl DatabaseServerStoredReferenceTrait for DatabaseServerReference {
    type Effects = ();
    type Requests = ();

    fn cf_name(&self) -> String {
        todo!()
    }

    fn get_database_server_stored(&self, requests: &Self::Requests) -> DbResult<DataBaseServerStored> {
        todo!()
    }

    fn put_database_server_stored(data_base_server: DataBaseServerStored) -> DbResult<Self::Effects> {
        todo!()
    }

    fn post_database_server_stored(data_base_server: DataBaseServerStored) -> DbResult<Self::Effects> {
        todo!()
    }

    fn delete_database_server_stored(&self) -> DbResult<Self::Effects> {
        todo!()
    }

    fn list_domain_names(&self, requests: &Self::Requests) -> DbResult<Vec<String>> {
        todo!()
    }
}


impl DatabaseServerReferenceTrait for DatabaseServerReference {
    type Effects = ();
    type Requests = ();

    fn cf_name(&self) -> String {
        todo!()
    }

    fn get_database_server(&self, requests: &Self::Requests) -> DbResult<DataBaseServer> {
        todo!()
    }

    fn put_database_server(
        data_base_server: DataBaseServer,
        requests: &Self::Requests,
    ) -> DbResult<Self::Effects> {
        todo!()
    }

    fn post_database_server(
        db_server_u: DataBaseServer,
        requests: &Self::Requests,
    ) -> DbResult<Self::Effects> {
        todo!()
    }

    fn delete_database_server(&self) -> DbResult<Self::Effects> {
        todo!()
    }

    fn list_domain_names(&self, requests: &Self::Requests) -> DbResult<Vec<String>> {
        todo!()
    }
}
