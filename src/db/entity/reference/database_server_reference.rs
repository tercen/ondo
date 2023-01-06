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
    fn list_domain_names_(&self, requests: &dyn Requests) -> DbResult<Vec<String>>;
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

    fn list_domain_names_(&self, requests: &dyn Requests) -> DbResult<Vec<String>> {
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

        fn create_ref() -> DatabaseServerReference {
            DatabaseServerReference::new()
        }

        fn create_stored() -> DatabaseServerStored {
            DatabaseServerStored {
                database_server: DatabaseServer,
                domains: HashMap::new(),
            }
        }

        #[test]
        fn test_cf_name() {
            let ref_trait = DatabaseServerReference;
            let expected_name = CfName::for_server_meta();

            assert_eq!(ref_trait.cf_name(), expected_name);
        }

        #[test]
        fn test_get_database_server_stored_failure() {
            let mut mock = MockTestRequests::new();
            let ref_trait = create_ref();
            mock.expect_get_database_server_stored()
                .returning(|_, _| Err(DbError::DatabaseNotInitialized));

            assert_eq!(
                ref_trait.get_database_server_stored(&mock).unwrap_err(),
                DbError::DatabaseNotInitialized,
                "get_database_server_stored should return DbError::DatabaseNotInitialized if the key does not exist"
            );
        }

        #[test]
        fn test_get_database_server_stored_success() {
            let mut mock = MockTestRequests::new();
            let ref_trait = create_ref();
            let database_server_stored = create_stored();
            let boxed_stored = database_server_stored.clone();
            mock.expect_get_database_server_stored()
                .returning(move |_, _| Ok(Some(boxed_stored.clone())));
            assert_eq!(
                ref_trait.get_database_server_stored(&mock).unwrap(),
                Some(database_server_stored.clone()),
                "get_database_server_stored should return the stored value if the key exists"
            );
        }

        #[test]
        fn test_put_database_server_stored() {
            let ref_trait = create_ref();
            let data_base_server_stored = create_stored();

            let expected_effects = vec![DatabaseServerStoredEffect::Put(
                ref_trait.cf_name(),
                ref_trait.clone(),
                data_base_server_stored.clone(),
            )];

            let effects = ref_trait
                .put_database_server_stored(&data_base_server_stored)
                .unwrap();
            assert_eq!(effects, expected_effects);
        }

        #[test]
        fn test_post_database_server_stored() {
            let ref_trait = create_ref();
            let data_base_server_stored = create_stored();

            let expected_effects = vec![
                DatabaseServerStoredEffect::CreateCf(ref_trait.cf_name()),
                DatabaseServerStoredEffect::Put(
                    ref_trait.cf_name(),
                    ref_trait.clone(),
                    data_base_server_stored.clone(),
                ),
            ];

            let effects = ref_trait
                .post_database_server_stored(&data_base_server_stored)
                .unwrap();
            assert_eq!(effects, expected_effects);
        }

        #[test]
        fn test_delete_database_server_stored() {
            let ref_trait = DatabaseServerReference;
            let expected_effects = vec![DatabaseServerStoredEffect::Delete(
                ref_trait.cf_name(),
                ref_trait.clone(),
            )];

            let effects = ref_trait.delete_database_server_stored().unwrap();
            assert_eq!(effects, expected_effects);
        }

        #[test]
        fn test_list_domain_names_success() {
            let ref_trait = create_ref();
            let stored = create_stored();
            let mut mock = MockTestRequests::new();
        
            let stored_clone = stored.clone();
            mock.expect_get_database_server_stored()
                .returning(move |_, _| Ok(Some(stored_clone.clone())));
        
            let domain_names = ref_trait.list_domain_names_(&mock).unwrap();
            assert_eq!(domain_names, stored.domains.keys().cloned().collect::<Vec<_>>());
        }
        
    }
}
