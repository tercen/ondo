//domain_reference/stored.rs
//TODO: validate table name
//TEST: test cascade_delete

use super::*;
use crate::db::entity::reference::effect::domain_stored_effect::DomainStoredEffect;
use crate::db::entity::reference::requests::database_server_stored_requests::DatabaseServerStoredRequests;
use crate::db::entity::reference::requests::domain_stored_requests::DomainStoredRequests;
use crate::db::entity::reference::requests::table_stored_requests::TableStoredRequests;
use crate::db::entity::reference::table_reference::stored::TableStoredReferenceTrait;
use crate::db::entity::reference::{
    database_server_reference::stored::*, table_reference::TableReference,
};

pub(crate) trait DomainStoredReferenceTrait {
    fn container_cf_name(&self) -> String;
    fn required_cf_names(&self) -> Vec<String>;
    fn get_domain_stored(
        &self,
        requests: &dyn DomainStoredRequests,
    ) -> DbResult<Option<DomainStored>>;
    fn put_domain_stored(&self, domain_stored: &DomainStored) -> DbResult<Effects>;
    fn post_domain_stored(
        &self,
        domain_stored: &DomainStored,
        parent_requests: &dyn DatabaseServerStoredRequests,
    ) -> DbResult<Effects>;
    fn delete_domain_stored(
        &self,
        table_requests: &dyn TableStoredRequests,
        domain_requests: &dyn DomainStoredRequests,
        parent_requests: &dyn DatabaseServerStoredRequests,
    ) -> DbResult<Effects>;
    fn list_table_names_(&self, requests: &dyn DomainStoredRequests) -> DbResult<Vec<String>>;
}

impl DomainStoredReferenceTrait for DomainReference {
    fn container_cf_name(&self) -> String {
        CfNameMaker::for_domain_meta()
    }

    fn required_cf_names(&self) -> Vec<String> {
        vec![
            CfNameMaker::for_table_meta(self),
            self.cf_name_for_table_counters(),
        ]
    }

    fn get_domain_stored(
        &self,
        requests: &dyn DomainStoredRequests,
    ) -> DbResult<Option<DomainStored>> {
        let key = self;
        requests.get_domain_stored(&self.container_cf_name(), &key.domain_name)
    }

    fn put_domain_stored(&self, domain_stored: &DomainStored) -> DbResult<Effects> {
        let effect = Effect::DomainStoredEffect(DomainStoredEffect::Put(
            self.container_cf_name(),
            self.domain_name.clone(),
            (*domain_stored).clone(),
        ));
        Ok(vec![effect])
    }

    fn post_domain_stored(
        &self,
        domain_stored: &DomainStored,
        parent_requests: &dyn DatabaseServerStoredRequests,
    ) -> DbResult<Effects> {
        let database_server_reference = self.to_database_server_reference();

        let database_server_stored_opt =
            database_server_reference.get_database_server_stored(parent_requests)?;
        let mut database_server_stored =
            database_server_stored_opt.ok_or(DbError::DatabaseNotInitialized)?;

        let mut effects = self
            .required_cf_names()
            .into_iter()
            .map(|cf_name| Effect::CreateCf(cf_name))
            .collect::<Vec<_>>();

        database_server_stored
            .domains
            .insert(self.domain_name.clone(), ());

        effects
            .extend(database_server_reference.put_database_server_stored(&database_server_stored)?);

        effects.extend(self.put_domain_stored(domain_stored)?);
        Ok(effects)
    }

    fn delete_domain_stored(
        &self,
        table_requests: &dyn TableStoredRequests,
        requests: &dyn DomainStoredRequests,
        parent_requests: &dyn DatabaseServerStoredRequests,
    ) -> DbResult<Effects> {
        fn cascade_delete_domain_stored(
            domain_reference: &DomainReference,
            table_requests: &dyn TableStoredRequests,
            requests: &dyn DomainStoredRequests,
        ) -> DbResult<Effects> {
            let mut effects = vec![];
            let table_names = domain_reference.list_table_names_(requests)?;
            for table_name in table_names {
                let table_reference = TableReference {
                    domain_reference: domain_reference.clone(),
                    table_name: table_name.clone(),
                };
                effects.extend(table_reference.delete_table_stored(table_requests, requests)?);
            }
            Ok(effects)
        }

        let mut effects = cascade_delete_domain_stored(self, table_requests, requests)?;

        let database_server_reference = self.to_database_server_reference();

        let database_server_stored_opt =
            database_server_reference.get_database_server_stored(parent_requests)?;
        let mut database_server_stored =
            database_server_stored_opt.ok_or(DbError::DatabaseNotInitialized)?;
        database_server_stored.domains.remove(&self.domain_name);
        effects
            .extend(database_server_reference.put_database_server_stored(&database_server_stored)?);

        effects.push(Effect::DomainStoredEffect(DomainStoredEffect::Delete(
            self.container_cf_name(),
            self.domain_name.clone(),
        )));
        effects.extend(
            self.required_cf_names()
                .into_iter()
                .map(|cf_name| Effect::DeleteCf(cf_name)),
        );
        Ok(effects)
    }

    fn list_table_names_(&self, requests: &dyn DomainStoredRequests) -> DbResult<Vec<String>> {
        let domain_stored_opt = self.get_domain_stored(requests)?;
        let domain_stored = domain_stored_opt.ok_or(DbError::DomainNotInitialized)?;
        let mut table_names = domain_stored.tables.keys().cloned().collect::<Vec<_>>();
        table_names.sort();
        Ok(table_names)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::db::entity::reference::database_server_reference::stored::tests::{
        create_database_server_stored, MockDatabaseServerStoredTestRequests,
    };
    use crate::db::entity::reference::table_reference::stored::tests::{
        create_table_stored, MockTableStoredTestRequests,
    };
    use crate::db::entity::{DatabaseServer, DatabaseServerStored};
    use mockall::*;
    use std::collections::HashMap;

    mock! {
        pub(crate) DomainStoredTestRequests {}
        impl DomainStoredRequests for DomainStoredTestRequests {
            fn get_domain_stored(
                &self,
                cf_name: &str,
                key: &DomainName,
            ) -> DbResult<Option<DomainStored>>;        }
    }

    pub(crate) fn create_domain_ref() -> DomainReference {
        DomainReference::new("sample_domain")
    }

    pub(crate) fn create_domain() -> Domain {
        Domain {
            reference: create_domain_ref(),
        }
    }

    pub(crate) fn create_domain_stored() -> DomainStored {
        DomainStored {
            domain: create_domain(),
            tables: HashMap::new(),
        }
    }

    mod domain_stored_reference_trait {
        use super::*;
        use crate::db::entity::reference::effect::database_server_stored_effect::DatabaseServerStoredEffect;

        #[test]
        fn test_get_domain_stored_failure() {
            let mut mock = MockDomainStoredTestRequests::new();
            let ref_trait = create_domain_ref();
            mock.expect_get_domain_stored().returning(|_, _| Ok(None));

            assert_eq!(
                ref_trait.get_domain_stored(&mock),
                Ok(None),
                "get_domain_stored should return Ok(None) if the key does not exist"
            );
        }

        #[test]
        fn test_get_domain_stored_success() {
            let mut mock = MockDomainStoredTestRequests::new();
            let ref_trait = create_domain_ref();
            let domain_stored = create_domain_stored();
            let boxed_stored = domain_stored.clone();
            mock.expect_get_domain_stored()
                .returning(move |_, _| Ok(Some(boxed_stored.clone())));
            assert_eq!(
                ref_trait.get_domain_stored(&mock).unwrap(),
                Some(domain_stored.clone()),
                "get_domain_stored should return the stored value if the key exists"
            );
        }

        #[test]
        fn test_put_domain_stored() {
            let ref_trait = create_domain_ref();
            let domain_stored = create_domain_stored();

            let expected_effects = vec![Effect::DomainStoredEffect(DomainStoredEffect::Put(
                ref_trait.container_cf_name(),
                ref_trait.domain_name.clone(),
                domain_stored.clone(),
            ))];

            let effects = ref_trait.put_domain_stored(&domain_stored).unwrap();
            assert_eq!(effects, expected_effects);
        }

        #[test]
        fn test_post_domain_stored() {
            let ref_trait = create_domain_ref();
            let domain_stored = create_domain_stored();

            let expected_effects = vec![
                Effect::CreateCf("/domains/sample_domain/tables".to_owned()),
                Effect::CreateCf("/domains/sample_domain/counters".to_owned()),
                Effect::DatabaseServerStoredEffect(DatabaseServerStoredEffect::Put(
                    "/server".to_owned(),
                    (),
                    DatabaseServerStored {
                        meta_revision: 0,
                        database_server: DatabaseServer::default(),
                        domains: {
                            vec!["sample_domain".to_owned()]
                                .into_iter()
                                .map(|s| (s, ()))
                                .collect()
                        },
                    },
                )),
                Effect::DomainStoredEffect(DomainStoredEffect::Put(
                    "/domains".to_owned(),
                    "sample_domain".to_owned(),
                    DomainStored {
                        domain: Domain {
                            reference: DomainReference {
                                domain_name: "sample_domain".to_owned(),
                            },
                        },
                        tables: HashMap::new(),
                    },
                )),
            ];

            let mut parent_mock = MockDatabaseServerStoredTestRequests::new();
            parent_mock
                .expect_get_database_server_stored()
                .returning(move |_, _| Ok(Some(create_database_server_stored())));

            let effects = ref_trait
                .post_domain_stored(&domain_stored, &parent_mock)
                .unwrap();
            assert_eq!(effects, expected_effects);
        }

        #[test]
        fn test_delete_domain_stored() {
            let ref_trait = DomainReference::new("sample_domain");

            let expected_effects = vec![
                Effect::DatabaseServerStoredEffect(DatabaseServerStoredEffect::Put(
                    "/server".to_owned(),
                    (),
                    DatabaseServerStored {
                        meta_revision: 0,
                        database_server: DatabaseServer::default(),
                        domains: HashMap::new(),
                    },
                )),
                Effect::DomainStoredEffect(DomainStoredEffect::Delete(
                    "/domains".to_owned(),
                    "sample_domain".to_owned(),
                )),
                Effect::DeleteCf("/domains/sample_domain/tables".to_owned()),
                Effect::DeleteCf("/domains/sample_domain/counters".to_owned()),
            ];

            let mut table_mock = MockTableStoredTestRequests::new();
            table_mock
                .expect_get_table_stored()
                .returning(move |_, _| Ok(Some(create_table_stored())));

            let mut domain_mock = MockDomainStoredTestRequests::new();
            domain_mock
                .expect_get_domain_stored()
                .returning(move |_, _| Ok(Some(create_domain_stored())));

            let mut parent_mock = MockDatabaseServerStoredTestRequests::new();
            parent_mock
                .expect_get_database_server_stored()
                .returning(move |_, _| Ok(Some(create_database_server_stored())));

            let effects = ref_trait
                .delete_domain_stored(&table_mock, &domain_mock, &parent_mock)
                .unwrap();
            assert_eq!(effects, expected_effects);
        }

        #[test]
        fn test_list_table_names_success() {
            let ref_trait = create_domain_ref();
            let stored = create_domain_stored();
            let mut mock = MockDomainStoredTestRequests::new();

            let stored_clone = stored.clone();
            mock.expect_get_domain_stored()
                .returning(move |_, _| Ok(Some(stored_clone.clone())));

            let table_names = ref_trait.list_table_names_(&mock).unwrap();
            assert_eq!(
                table_names,
                stored.tables.keys().cloned().collect::<Vec<_>>()
            );
        }
    }
}
