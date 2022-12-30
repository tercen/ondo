use super::super::DatabaseServer;
use super::super::DatabaseServerStored;
use super::super::DbError;
use super::super::DbResult;
use super::CfName;

pub trait DatabaseServerStoredRequests {
    fn get_database_server_stored(
        &self,
        cf_name: &str,
        key: &DatabaseServerReference,
    ) -> DbResult<Option<DatabaseServerStored>>;
}

pub enum DatabaseServerStoredEffect {
    CreateCf(String),
    Put(String, DatabaseServerReference, DatabaseServerStored),
    Delete(String, DatabaseServerReference),
}

pub type Effects = Vec<DatabaseServerStoredEffect>;
pub trait Requests: DatabaseServerStoredRequests {}

pub(super) trait DatabaseServerStoredReferenceTrait {
    fn cf_name(&self) -> String;
    fn get_database_server_stored(
        &self,
        requests: &dyn Requests,
    ) -> DbResult<Option<DatabaseServerStored>>;
    fn put_database_server_stored(
        &self,
        data_base_server: DatabaseServerStored,
    ) -> DbResult<Effects>;
    fn post_database_server_stored(
        &self,
        data_base_server: DatabaseServerStored,
    ) -> DbResult<Effects>;
    fn delete_database_server_stored(&self) -> DbResult<Effects>;
    fn list_domain_names(&self, requests: &dyn Requests) -> DbResult<Vec<String>>;
}

pub trait DatabaseServerReferenceTrait {
    fn get_database_server(&self, requests: &dyn Requests) -> DbResult<Option<DatabaseServer>>;
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

    fn get_database_server_stored(
        &self,
        requests: &dyn Requests,
    ) -> DbResult<Option<DatabaseServerStored>> {
        let key = self;
        requests.get_database_server_stored(&self.cf_name(), key)
    }

    fn put_database_server_stored(
        &self,
        data_base_server: DatabaseServerStored,
    ) -> DbResult<Effects> {
        let effects = vec![DatabaseServerStoredEffect::Put(
            self.cf_name(),
            self.clone(),
            data_base_server,
        )];
        Ok(effects)
    }

    fn post_database_server_stored(
        &self,
        data_base_server: DatabaseServerStored,
    ) -> DbResult<Effects> {
        let cf_name = self.cf_name();
        let mut effects = vec![DatabaseServerStoredEffect::CreateCf(cf_name)];
        effects.extend(self.put_database_server_stored(data_base_server)?);
        Ok(effects)
    }

    fn delete_database_server_stored(&self) -> DbResult<Effects> {
        let effects = vec![DatabaseServerStoredEffect::Delete(
            self.cf_name(),
            self.clone(),
        )];
        Ok(effects)
    }

    fn list_domain_names(&self, requests: &dyn Requests) -> DbResult<Vec<String>> {
        let database_server_stored = self.get_database_server_stored(requests)?;
        if let Some(database_server_stored) = database_server_stored {
            Ok(database_server_stored.domains.keys().cloned().collect())
        } else {
            Err(DbError::DatabaseNotInitialized)
        }
    }
}

impl DatabaseServerReferenceTrait for DatabaseServerReference {
    fn get_database_server(&self, requests: &dyn Requests) -> DbResult<Option<DatabaseServer>> {
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

