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

#[derive(Debug, Clone, PartialEq, Eq)]
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
        data_base_server: &DatabaseServerStored,
    ) -> DbResult<Effects>;
    fn post_database_server_stored(
        &self,
        data_base_server: &DatabaseServerStored,
    ) -> DbResult<Effects>;
    fn delete_database_server_stored(&self) -> DbResult<Effects>;
    fn list_domain_names(&self, requests: &dyn Requests) -> DbResult<Vec<String>>;
}

pub trait DatabaseServerReferenceTrait {
    fn get_database_server(&self, requests: &dyn Requests) -> DbResult<Option<DatabaseServer>>;
    fn put_database_server(
        data_base_server: &DatabaseServer,
        requests: &dyn Requests,
    ) -> DbResult<Effects>;
    fn post_database_server(
        data_base_server: &DatabaseServer,
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
        match requests.get_database_server_stored(&self.cf_name(), key) {
            Ok(Some(database_server_stored)) => Ok(Some(database_server_stored)),
            Ok(None) => Err(DbError::DatabaseNotInitialized),
            Err(e) => Err(e),
        }
    }

    fn put_database_server_stored(
        &self,
        data_base_server: &DatabaseServerStored,
    ) -> DbResult<Effects> {
        let effects = vec![DatabaseServerStoredEffect::Put(
            self.cf_name(),
            self.clone(),
            (*data_base_server).clone(),
        )];
        Ok(effects)
    }

    fn post_database_server_stored(
        &self,
        data_base_server: &DatabaseServerStored,
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
        data_base_server: &DatabaseServer,
        requests: &dyn Requests,
    ) -> DbResult<Effects> {
        todo!()
    }

    fn post_database_server(
        data_base_server: &DatabaseServer,
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

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::*;
    use std::collections::HashMap;

    mock! {
        TestRequests {}
        impl Requests for TestRequests {}
        impl DatabaseServerStoredRequests for TestRequests {
            fn get_database_server_stored(
                &self,
                cf_name: &str,
                key: &DatabaseServerReference,
            ) -> DbResult<Option<DatabaseServerStored>>;        }
    }

    mod database_server_stored_reference_trait {
        use super::*;

        #[test]
        fn test_get_database_server_stored() {
            let mut mock = MockTestRequests::new();
            let ref_ = DatabaseServerReference::new();

            let stored = DatabaseServerStored {
                database_server: DatabaseServer,
                domains: HashMap::new(),
            };

            mock.expect_get_database_server_stored()
                .returning(|_, _| Err(DbError::DatabaseNotInitialized));

            // Test get_database_server_stored
            assert_eq!(
                ref_.get_database_server_stored(&mock).unwrap_err(),
                DbError::DatabaseNotInitialized,
                "get_database_server_stored should return DbError::DatabaseNotInitialized if the key does not exist"
            );

            let boxed_stored = stored.clone();
            let mut mock2 = MockTestRequests::new();
            mock2
                .expect_get_database_server_stored()
                .returning(move |_, _| Ok(Some(boxed_stored.clone())));
            assert_eq!(
                ref_.get_database_server_stored(&mock2).unwrap(),
                Some(stored.clone()),
                "get_database_server_stored should return the stored value if the key exists"
            );
        }

        #[test]
        fn test_put_database_server_stored() {
            let ref_trait = DatabaseServerReference;
            let data_base_server_stored = DatabaseServerStored {
                database_server: DatabaseServer,
                domains: HashMap::new(),
            };
            let expected_effects = vec![DatabaseServerStoredEffect::Put(
                ref_trait.cf_name(),
                ref_trait.clone(),
                data_base_server_stored.clone(),
            )];

            let effects = ref_trait.put_database_server_stored(&data_base_server_stored).unwrap();
            assert_eq!(effects, expected_effects);
        }
        
        #[test]
        fn test_post_database_server_stored() {
            let ref_trait = DatabaseServerReference;
            let data_base_server_stored = DatabaseServerStored {
                database_server: DatabaseServer,
                domains: HashMap::new(),
            };
            let expected_effects = vec![
                DatabaseServerStoredEffect::CreateCf(ref_trait.cf_name()),
                DatabaseServerStoredEffect::Put(
                    ref_trait.cf_name(),
                    ref_trait.clone(),
                    data_base_server_stored.clone(),
                ),
            ];
        
            let effects = ref_trait.post_database_server_stored(&data_base_server_stored).unwrap();
            assert_eq!(effects, expected_effects);
        }
        

    }
}
