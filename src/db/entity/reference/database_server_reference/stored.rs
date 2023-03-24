//database_server_reference/stored.rs
//TEST: test cascade_delete

use super::*;
use crate::db::entity::reference::effect::database_server_stored_effect::DatabaseServerStoredEffect;
use crate::db::entity::reference::requests::database_server_stored_requests::DatabaseServerStoredRequests;
use crate::db::entity::reference::requests::domain_stored_requests::DomainStoredRequests;
use crate::db::entity::reference::{domain_reference::DomainReferenceTrait, DomainReference};

pub(crate) trait DatabaseServerStoredReferenceTrait {
    fn container_cf_name(&self) -> String;
    fn required_cf_names(&self) -> Vec<String>;
    fn get_database_server_stored(
        &self,
        requests: &dyn DatabaseServerStoredRequests,
    ) -> DbResult<Option<DatabaseServerStored>>;
    fn put_database_server_stored(
        &self,
        database_server: &DatabaseServerStored,
    ) -> DbResult<Effects>;
    fn post_database_server_stored(
        &self,
        database_server: &DatabaseServerStored,
    ) -> DbResult<Effects>;
    fn delete_database_server_stored(
        &self,
        table_requests: &dyn TableStoredRequests,
        domain_requests: &dyn DomainStoredRequests,
        requests: &dyn DatabaseServerStoredRequests,
    ) -> DbResult<Effects>;
    fn list_domain_names_(
        &self,
        requests: &dyn DatabaseServerStoredRequests,
    ) -> DbResult<Vec<String>>;
}

impl DatabaseServerStoredReferenceTrait for DatabaseServerReference {
    fn container_cf_name(&self) -> String {
        CfNameMaker::for_server_meta()
    }
    fn required_cf_names(&self) -> Vec<String> {
        vec![self.container_cf_name(), CfNameMaker::for_domain_meta()]
    }

    fn get_database_server_stored(
        &self,
        requests: &dyn DatabaseServerStoredRequests,
    ) -> DbResult<Option<DatabaseServerStored>> {
        requests.get_database_server_stored(&self.container_cf_name(), &())
    }

    fn put_database_server_stored(
        &self,
        database_server: &DatabaseServerStored,
    ) -> DbResult<Effects> {
        let effects = vec![Effect::DatabaseServerStoredEffect(
            DatabaseServerStoredEffect::Put(
                self.container_cf_name(),
                (),
                (*database_server).clone(),
            ),
        )];
        Ok(effects)
    }

    fn post_database_server_stored(
        &self,
        database_server: &DatabaseServerStored,
    ) -> DbResult<Effects> {
        let mut effects = self
            .required_cf_names()
            .iter()
            .map(|cf_name| Effect::CreateCf(cf_name.clone()))
            .collect::<Vec<_>>();
        effects.extend(self.put_database_server_stored(database_server)?);
        Ok(effects)
    }

    fn delete_database_server_stored(
        &self,
        table_requests: &dyn TableStoredRequests,
        domain_requests: &dyn DomainStoredRequests,
        requests: &dyn DatabaseServerStoredRequests,
    ) -> DbResult<Effects> {
        fn cascade_delete_database_server_stored(
            database_server_reference: &DatabaseServerReference,
            table_requests: &dyn TableStoredRequests,
            domain_requests: &dyn DomainStoredRequests,
            requests: &dyn DatabaseServerStoredRequests,
        ) -> DbResult<Effects> {
            let mut effects = vec![];
            let domain_names = database_server_reference.list_domain_names_(requests)?;
            for domain_name in domain_names {
                let domain_reference = DomainReference {
                    domain_name: domain_name.clone(),
                };
                effects.extend(domain_reference.delete_domain(
                    table_requests,
                    domain_requests,
                    requests,
                )?);
            }
            Ok(effects)
        }

        let mut effects =
            cascade_delete_database_server_stored(self, table_requests, domain_requests, requests)?;

        effects.extend(vec![Effect::DatabaseServerStoredEffect(
            DatabaseServerStoredEffect::Delete(self.container_cf_name(), ()),
        )]);
        effects.extend(
            self.required_cf_names()
                .iter()
                .map(|cf_name| Effect::DeleteCf(cf_name.clone())),
        );

        Ok(effects)
    }

    fn list_domain_names_(
        &self,
        requests: &dyn DatabaseServerStoredRequests,
    ) -> DbResult<Vec<String>> {
        let database_server_stored = self.get_database_server_stored(requests)?;
        if let Some(database_server_stored) = database_server_stored {
            let mut keys = database_server_stored
                .domains
                .keys()
                .cloned()
                .collect::<Vec<String>>();
            keys.sort_unstable();
            Ok(keys)
        } else {
            Err(DbError::DatabaseNotInitialized)
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::db::entity::reference::domain_reference::stored::tests::{
        create_domain_stored, MockDomainStoredTestRequests,
    };
    use crate::db::entity::reference::table_reference::stored::tests::{
        create_table_stored, MockTableStoredTestRequests,
    };
    use mockall::*;
    use std::collections::HashMap;

    mock! {
        pub(crate) DatabaseServerStoredTestRequests {}
        impl DatabaseServerStoredRequests for DatabaseServerStoredTestRequests {
            fn get_database_server_stored(
                &self,
                cf_name: &str,
                key: &DatabaseServerName,
            ) -> DbResult<Option<DatabaseServerStored>>;        }
    }

    pub(crate) fn create_database_server_ref() -> DatabaseServerReference {
        DatabaseServerReference::default()
    }

    pub(crate) fn create_database_server() -> DatabaseServer {
        DatabaseServer::default()
    }

    pub(crate) fn create_database_server_stored() -> DatabaseServerStored {
        DatabaseServerStored {
            meta_revision: 0,
            database_server: create_database_server(),
            domains: HashMap::new(),
        }
    }

    mod database_server_stored_reference_trait {
        use super::*;

        #[test]
        fn test_get_database_server_stored_failure() {
            let mut mock = MockDatabaseServerStoredTestRequests::new();
            let ref_trait = create_database_server_ref();
            mock.expect_get_database_server_stored()
                .returning(|_, _| Ok(None));

            assert_eq!(
                ref_trait.get_database_server_stored(&mock),
                Ok(None),
                "get_database_server_stored should return Ok(None) if the key does not exist"
            );
        }

        #[test]
        fn test_get_database_server_stored_success() {
            let mut mock = MockDatabaseServerStoredTestRequests::new();
            let ref_trait = create_database_server_ref();
            let database_server_stored = create_database_server_stored();
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
            let ref_trait = create_database_server_ref();
            let database_server_stored = create_database_server_stored();

            let expected_effects = vec![Effect::DatabaseServerStoredEffect(
                DatabaseServerStoredEffect::Put(
                    ref_trait.container_cf_name(),
                    (),
                    database_server_stored.clone(),
                ),
            )];

            let effects = ref_trait
                .put_database_server_stored(&database_server_stored)
                .unwrap();
            assert_eq!(effects, expected_effects);
        }

        #[test]
        fn test_post_database_server_stored() {
            let ref_trait = create_database_server_ref();
            let database_server_stored = create_database_server_stored();

            let expected_effects = vec![
                Effect::CreateCf(CfNameMaker::for_server_meta()),
                Effect::CreateCf(CfNameMaker::for_domain_meta()),
                Effect::DatabaseServerStoredEffect(DatabaseServerStoredEffect::Put(
                    ref_trait.container_cf_name(),
                    (),
                    database_server_stored.clone(),
                )),
            ];

            let effects = ref_trait
                .post_database_server_stored(&database_server_stored)
                .unwrap();
            assert_eq!(effects, expected_effects);
        }

        #[test]
        fn test_delete_database_server_stored() {
            let mut table_mock = MockTableStoredTestRequests::new();
            table_mock
                .expect_get_table_stored()
                .returning(move |_, _| Ok(Some(create_table_stored())));

            let mut domain_mock = MockDomainStoredTestRequests::new();
            domain_mock
                .expect_get_domain_stored()
                .returning(move |_, _| Ok(Some(create_domain_stored())));
            let mut database_server_mock = MockDatabaseServerStoredTestRequests::new();
            database_server_mock
                .expect_get_database_server_stored()
                .returning(move |_, _| Ok(Some(create_database_server_stored())));

            let ref_trait = DatabaseServerReference;

            let mut expected_effects = vec![Effect::DatabaseServerStoredEffect(
                DatabaseServerStoredEffect::Delete(ref_trait.container_cf_name(), ()),
            )];
            expected_effects.extend(
                ref_trait
                    .required_cf_names()
                    .iter()
                    .map(|cf_name| Effect::DeleteCf(cf_name.clone())),
            );

            let effects = ref_trait
                .delete_database_server_stored(&table_mock, &domain_mock, &database_server_mock)
                .unwrap();
            assert_eq!(effects, expected_effects);
        }

        #[test]
        fn test_list_domain_names_success() {
            let ref_trait = create_database_server_ref();
            let stored = create_database_server_stored();
            let mut mock = MockDatabaseServerStoredTestRequests::new();

            let stored_clone = stored.clone();
            mock.expect_get_database_server_stored()
                .returning(move |_, _| Ok(Some(stored_clone.clone())));

            let domain_names = ref_trait.list_domain_names_(&mock).unwrap();
            assert_eq!(
                domain_names,
                stored.domains.keys().cloned().collect::<Vec<_>>()
            );
        }
    }
}
