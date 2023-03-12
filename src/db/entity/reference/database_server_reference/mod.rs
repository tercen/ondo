//database_server_reference.rs
use crate::db::entity::reference::requests::database_server_stored_requests::DatabaseServerStoredRequests;
use crate::db::entity::reference::requests::domain_stored_requests::DomainStoredRequests;
use crate::db::entity::reference::requests::table_stored_requests::TableStoredRequests;
use crate::db::{
    db_error::{DbError, DbResult},
    entity::{
        reference::{
            effect::{Effect, Effects},
            CfNameMaker,
        },
        DatabaseServer, DatabaseServerStored,
    },
};

pub(crate) mod stored;
use stored::*;

pub(crate) trait DatabaseServerReferenceTrait {
    fn get_database_server(
        &self,
        requests: &dyn DatabaseServerStoredRequests,
    ) -> DbResult<Option<DatabaseServer>>;
    fn put_database_server(
        &self,
        database_server: &DatabaseServer,
        requests: &dyn DatabaseServerStoredRequests,
    ) -> DbResult<Effects>;
    fn post_database_server(
        &self,
        database_server: &DatabaseServer,
        requests: &dyn DatabaseServerStoredRequests,
    ) -> DbResult<Effects>;
    fn delete_database_server(
        &self,
        table_requests: &dyn TableStoredRequests,
        domain_requests: &dyn DomainStoredRequests,
        requests: &dyn DatabaseServerStoredRequests,
    ) -> DbResult<Effects>;
    fn list_domain_names(
        &self,
        requests: &dyn DatabaseServerStoredRequests,
    ) -> DbResult<Vec<String>>;
}

pub type DatabaseServerName = ();
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatabaseServerReference;

impl DatabaseServerReference {
    pub fn new() -> Self {
        DatabaseServerReference
    }
}

impl DatabaseServerReferenceTrait for DatabaseServerReference {
    // Gets a DatabaseServer from the database.
    fn get_database_server(
        &self,
        requests: &dyn DatabaseServerStoredRequests,
    ) -> DbResult<Option<DatabaseServer>> {
        self.get_database_server_stored(requests)
            .map(|opt| opt.map(|database_server_stored| database_server_stored.database_server))
    }

    // Updates a DatabaseServer in the database.
    fn put_database_server(
        &self,
        database_server: &DatabaseServer,
        requests: &dyn DatabaseServerStoredRequests,
    ) -> DbResult<Effects> {
        let stored_opt = self.get_database_server_stored(requests)?;
        match stored_opt {
            None => return Err(DbError::DatabaseNotInitialized),
            Some(stored) => {
                let mut new_stored = stored.clone();
                new_stored.database_server = (*database_server).clone();
                self.put_database_server_stored(&new_stored)
            }
        }
    }
    // Creates a DatabaseServer in the database.
    fn post_database_server(
        &self,
        database_server: &DatabaseServer,
        requests: &dyn DatabaseServerStoredRequests,
    ) -> DbResult<Effects> {
        let stored_opt = self.get_database_server_stored(requests)?;
        match stored_opt {
            Some(_) => {
                // Data already exists, return AlreadyExists error
                Err(DbError::AlreadyExists)
            }
            None => {
                let new_stored = DatabaseServerStored {
                    meta_revision: 0,
                    database_server: (*database_server).clone(),
                    domains: Default::default(),
                };
                self.post_database_server_stored(&new_stored)
            }
        }
    }

    // Deletes a DatabaseServer from the database.
    fn delete_database_server(
        &self,
        table_requests: &dyn TableStoredRequests,
        domain_requests: &dyn DomainStoredRequests,
        requests: &dyn DatabaseServerStoredRequests,
    ) -> DbResult<Effects> {
        self.delete_database_server_stored(table_requests, domain_requests, requests)
    }

    // Lists the domain names of the DatabaseServer.
    fn list_domain_names(
        &self,
        requests: &dyn DatabaseServerStoredRequests,
    ) -> DbResult<Vec<String>> {
        self.list_domain_names_(requests)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::entity::reference::database_server_reference::stored::tests::{
        create_database_server, create_database_server_ref, create_database_server_stored,
        MockDatabaseServerStoredTestRequests,
    };

    mod database_server_reference_trait {
        use super::*;
        use crate::db::entity::reference::effect::database_server_stored_effect::DatabaseServerStoredEffect;
        #[test]
        fn test_get_database_server_failure() {
            let mut mock = MockDatabaseServerStoredTestRequests::new();
            let ref_trait = create_database_server_ref();
            mock.expect_get_database_server_stored()
                .returning(|_, _| Err(DbError::DatabaseNotInitialized));

            assert_eq!(
                ref_trait.get_database_server(&mock).unwrap_err(),
                DbError::DatabaseNotInitialized,
                "get_database_server should return DbError::DatabaseNotInitialized if the key does not exist"
            );
        }

        #[test]
        fn test_get_database_server_success() {
            let mut mock = MockDatabaseServerStoredTestRequests::new();
            let ref_trait = create_database_server_ref();
            let database_server = create_database_server();
            mock.expect_get_database_server_stored()
                .returning(move |_, _| Ok(Some(create_database_server_stored())));
            assert_eq!(
                ref_trait.get_database_server(&mock).unwrap(),
                Some(database_server.clone()),
                "get_database_server_stored should return the stored value if the key exists"
            );
        }

        #[test]
        fn test_put_database_server() {
            let mut mock = MockDatabaseServerStoredTestRequests::new();
            let ref_trait = create_database_server_ref();
            let database_server_stored = create_database_server_stored();

            mock.expect_get_database_server_stored()
                .returning(move |_, _| Ok(Some(create_database_server_stored())));

            let expected_effects = vec![Effect::DatabaseServerStoredEffect(
                DatabaseServerStoredEffect::Put(
                    ref_trait.container_cf_name(),
                    (),
                    database_server_stored.clone(),
                ),
            )];

            let effects = ref_trait
                .put_database_server(&database_server_stored.database_server, &mock)
                .unwrap();
            assert_eq!(effects, expected_effects);
        }

        #[test]
        fn test_list_domain_names() {
            let mut mock = MockDatabaseServerStoredTestRequests::new();
            let ref_trait = create_database_server_ref();

            let example_stored = DatabaseServerStored {
                meta_revision: 0,
                database_server: DatabaseServer,
                domains: vec![
                    ("example1.com".to_owned(), ()),
                    ("example2.com".to_owned(), ()),
                    ("example3.com".to_owned(), ()),
                ]
                .into_iter()
                .collect(),
            };

            let example_stored_clone = example_stored.clone();
            mock.expect_get_database_server_stored()
                .returning(move |_, _| Ok(Some(example_stored_clone.clone())));

            let domain_names = ref_trait.list_domain_names(&mock).unwrap();
            // domain_names.sort();
            assert_eq!(
                vec![
                    "example1.com".to_owned(),
                    "example2.com".to_owned(),
                    "example3.com".to_owned()
                ],
                domain_names
            );
        }
    }
}
