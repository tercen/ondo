//table_reference/stored.rs
//TODO: validate domain name
//TEST: test cascade_delete
use super::*;
use crate::db::{
    entity::{OndoKey, TableValue},
    reference::{
        domain_reference::stored::*,
        effect::TableStoredEffect,
        index_reference::*,
        requests::{DomainStoredRequests, TableStoredIteratorRequests, TableStoredRequests},
        IndexReference,
    },
    DbResult,
};

pub(crate) trait TableStoredReferenceTrait {
    fn container_cf_name(&self) -> String;
    fn required_cf_names(&self) -> Vec<String>;
    fn value_cf_name(&self) -> String;

    fn get_table_stored(&self, requests: &dyn TableStoredRequests)
        -> DbResult<Option<TableStored>>;
    fn put_table_stored(&self, table_stored: &TableStored) -> DbResult<Effects>;
    fn post_table_stored(
        &self,
        table_stored: &TableStored,
        parent_requests: &dyn DomainStoredRequests,
    ) -> DbResult<Effects>;
    fn delete_table_stored(
        &self,
        requests: &dyn TableStoredRequests,
        parent_requests: &dyn DomainStoredRequests,
    ) -> DbResult<Effects>;
    fn list_index_names_(&self, requests: &dyn TableStoredRequests) -> DbResult<Vec<String>>;
    fn all_values_<'a>(
        &self,
        requests: &'a dyn TableStoredIteratorRequests<'a>,
    ) -> DbResult<Box<dyn Iterator<Item = DbResult<TableValue>> + 'a>>;
    fn all_values_with_key_prefix_<'a>(
        &self,
        key_prefix: OndoKey,
        requests: &'a dyn TableStoredIteratorRequests<'a>,
    ) -> DbResult<Box<dyn Iterator<Item = DbResult<TableValue>> + 'a>>;
    fn all_values_with_key_range_<'a>(
        &self,
        start_key: OndoKey,
        end_key: OndoKey,
        requests: &'a dyn TableStoredIteratorRequests<'a>,
    ) -> DbResult<Box<dyn Iterator<Item = DbResult<TableValue>> + 'a>>;
}

impl TableStoredReferenceTrait for TableReference {
    fn container_cf_name(&self) -> String {
        CfNameMaker::for_table_meta(&self.to_domain_reference())
    }

    fn value_cf_name(&self) -> String {
        CfNameMaker::for_table_values(&self)
    }

    fn required_cf_names(&self) -> Vec<String> {
        vec![self.value_cf_name()]
    }

    fn all_values_<'a>(
        &self,
        requests: &'a dyn TableStoredIteratorRequests<'a>,
    ) -> DbResult<Box<dyn Iterator<Item = DbResult<TableValue>> + 'a>> {
        requests.all_values(&self.value_cf_name())
    }

    fn all_values_with_key_prefix_<'a>(
        &self,
        key_prefix: OndoKey,
        requests: &'a dyn TableStoredIteratorRequests<'a>,
    ) -> DbResult<Box<dyn Iterator<Item = DbResult<TableValue>> + 'a>> {
        requests.all_values_with_key_prefix(&self.value_cf_name(), key_prefix)
    }

    fn all_values_with_key_range_<'a>(
        &self,
        start_key: OndoKey,
        end_key: OndoKey,
        requests: &'a dyn TableStoredIteratorRequests<'a>,
    ) -> DbResult<Box<dyn Iterator<Item = DbResult<TableValue>> + 'a>> {
        requests.all_values_with_key_range(&self.value_cf_name(), start_key, end_key)
    }

    fn get_table_stored(
        &self,
        requests: &dyn TableStoredRequests,
    ) -> DbResult<Option<TableStored>> {
        let key = self;
        requests.get_table_stored(&self.container_cf_name(), &key.table_name)
    }

    fn put_table_stored(&self, table_stored: &TableStored) -> DbResult<Effects> {
        let effect = Effect::TableStoredEffect(TableStoredEffect::Put(
            self.container_cf_name(),
            self.table_name.clone(),
            (*table_stored).clone(),
        ));
        Ok(vec![effect])
    }

    fn post_table_stored(
        &self,
        table_stored: &TableStored,
        parent_requests: &dyn DomainStoredRequests,
    ) -> DbResult<Effects> {
        let domain_reference = self.to_domain_reference();

        let domain_stored_opt = domain_reference.get_domain_stored(parent_requests)?;
        let mut domain_stored = domain_stored_opt.ok_or(DbError::DomainNotInitialized)?;
        domain_stored.tables.insert(self.table_name.clone(), ());
        let mut effects = domain_reference.put_domain_stored(&domain_stored)?;

        effects.extend(
            self.required_cf_names()
                .iter()
                .map(|cf_name| Effect::CreateCf(cf_name.clone()))
                .collect::<Vec<_>>(),
        );
        effects.extend(self.put_table_stored(table_stored)?);
        Ok(effects)
    }

    fn delete_table_stored(
        &self,
        requests: &dyn TableStoredRequests,
        parent_requests: &dyn DomainStoredRequests,
    ) -> DbResult<Effects> {
        fn cascade_delete_table_stored(
            table_reference: &TableReference,
            requests: &dyn TableStoredRequests,
        ) -> DbResult<Effects> {
            let mut effects = vec![];
            let index_names = table_reference.list_index_names_(requests)?;
            for index_name in index_names {
                let index_reference = IndexReference {
                    index_name: index_name.clone(),
                    table_reference: table_reference.clone(),
                };
                effects.extend(index_reference.delete_index(requests)?);
            }
            Ok(effects)
        }

        let mut effects = cascade_delete_table_stored(self, requests)?;

        let domain_reference = self.to_domain_reference();

        let domain_stored_opt = domain_reference.get_domain_stored(parent_requests)?;
        let mut domain_stored = domain_stored_opt.ok_or(DbError::DomainNotInitialized)?;
        domain_stored.tables.remove(&self.table_name);
        effects.extend(domain_reference.put_domain_stored(&domain_stored)?);

        effects.push(Effect::TableStoredEffect(TableStoredEffect::Delete(
            self.container_cf_name(),
            self.table_name.clone(),
        )));

        effects.extend(
            self.required_cf_names()
                .iter()
                .map(|cf_name| Effect::DeleteCf(cf_name.clone())),
        );
        Ok(effects)
    }

    fn list_index_names_(&self, requests: &dyn TableStoredRequests) -> DbResult<Vec<String>> {
        let table_stored_opt = self.get_table_stored(requests)?;
        let table_stored = table_stored_opt.ok_or(DbError::TableNotInitialized)?;
        let mut index_names = table_stored.indexes.keys().cloned().collect::<Vec<_>>();
        index_names.sort();
        Ok(table_stored.indexes.keys().cloned().collect())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::db::entity::{Domain, DomainStored};
    use crate::db::reference::domain_reference::stored::tests::{
        create_domain_stored, MockDomainStoredTestRequests,
    };
    use mockall::*;
    use std::collections::HashMap;

    mock! {
        pub(crate) TableStoredTestRequests {}
        impl TableStoredRequests for TableStoredTestRequests {
            fn get_table_stored(
                &self,
                cf_name: &str,
                key: &TableName,
            ) -> DbResult<Option<TableStored>>;
            // fn all_values<'a>(&'a self, value_cf_name: &str) -> DbResult<Box<dyn Iterator<Item = DbResult<TableValue>>>>;
        }
    }

    pub(crate) fn create_table_ref() -> TableReference {
        TableReference::new("sample_domain", "sample_table")
    }

    pub(crate) fn create_table() -> Table {
        Table {
            reference: create_table_ref(),
        }
    }

    pub(crate) fn create_table_stored() -> TableStored {
        TableStored {
            table: create_table(),
            indexes: HashMap::new(),
        }
    }

    mod table_stored_reference_trait {
        use super::*;
        use crate::db::reference::effect::domain_stored_effect::DomainStoredEffect;
        #[test]
        fn test_get_table_stored_failure() {
            let mut mock = MockTableStoredTestRequests::new();
            let ref_trait = create_table_ref();
            mock.expect_get_table_stored().returning(|_, _| Ok(None));

            assert_eq!(
                ref_trait.get_table_stored(&mock),
                Ok(None),
                "get_table_stored should return Ok(None) if the key does not exist"
            );
        }

        #[test]
        fn test_get_table_stored_success() {
            let mut mock = MockTableStoredTestRequests::new();
            let ref_trait = create_table_ref();
            let table_stored = create_table_stored();
            let boxed_stored = table_stored.clone();
            mock.expect_get_table_stored()
                .returning(move |_, _| Ok(Some(boxed_stored.clone())));
            assert_eq!(
                ref_trait.get_table_stored(&mock).unwrap(),
                Some(table_stored.clone()),
                "get_table_stored should return the stored value if the key exists"
            );
        }

        #[test]
        fn test_put_table_stored() {
            let ref_trait = create_table_ref();
            let table_stored = create_table_stored();

            let expected_effects = vec![Effect::TableStoredEffect(TableStoredEffect::Put(
                "/domains/sample_domain/tables".to_owned(),
                "sample_table".to_owned(),
                TableStored {
                    table: Table {
                        reference: TableReference {
                            domain_reference: DomainReference::new("sample_domain"),
                            table_name: "sample_table".to_owned(),
                        },
                    },
                    indexes: HashMap::new(),
                },
            ))];

            let effects = ref_trait.put_table_stored(&table_stored).unwrap();
            assert_eq!(effects, expected_effects);
        }

        #[test]
        fn test_post_table_stored() {
            let ref_trait = create_table_ref();
            let table_stored = create_table_stored();

            let expected_effects = vec![
                Effect::DomainStoredEffect(DomainStoredEffect::Put(
                    "/domains".to_owned(),
                    "sample_domain".to_owned(),
                    DomainStored {
                        domain: Domain {
                            reference: DomainReference {
                                domain_name: "sample_domain".to_owned(),
                            },
                        },
                        tables: vec!["sample_table".to_owned()]
                            .into_iter()
                            .map(|s| (s, ()))
                            .collect(),
                    },
                )),
                Effect::CreateCf("sample_domain::/sample_table".to_owned()),
                Effect::TableStoredEffect(TableStoredEffect::Put(
                    "/domains/sample_domain/tables".to_owned(),
                    "sample_table".to_owned(),
                    TableStored {
                        table: Table {
                            reference: TableReference {
                                table_name: "sample_table".to_owned(),
                                domain_reference: DomainReference::new("sample_domain"),
                            },
                        },
                        indexes: HashMap::new(),
                    },
                )),
            ];

            let mut parent_mock = MockDomainStoredTestRequests::new();
            parent_mock
                .expect_get_domain_stored()
                .returning(move |_, _| Ok(Some(create_domain_stored())));

            let effects = ref_trait
                .post_table_stored(&table_stored, &parent_mock)
                .unwrap();
            assert_eq!(effects, expected_effects);
        }

        #[test]
        fn test_delete_table_stored() {
            let ref_trait = TableReference::new("sample_domain", "sample_table");

            let expected_effects = vec![
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
                Effect::TableStoredEffect(TableStoredEffect::Delete(
                    "/domains/sample_domain/tables".to_owned(),
                    "sample_table".to_owned(),
                )),
                Effect::DeleteCf("sample_domain::/sample_table".to_owned()),
            ];

            let mut mock = MockTableStoredTestRequests::new();
            mock.expect_get_table_stored()
                .returning(move |_, _| Ok(Some(create_table_stored())));

            let mut parent_mock = MockDomainStoredTestRequests::new();

            parent_mock
                .expect_get_domain_stored()
                .returning(move |_, _| Ok(Some(create_domain_stored())));

            let effects = ref_trait.delete_table_stored(&mock, &parent_mock).unwrap();
            assert_eq!(effects, expected_effects);
        }

        #[test]
        fn test_list_table_names_success() {
            let ref_trait = create_table_ref();
            let stored = create_table_stored();
            let mut mock = MockTableStoredTestRequests::new();

            let stored_clone = stored.clone();
            mock.expect_get_table_stored()
                .returning(move |_, _| Ok(Some(stored_clone.clone())));

            let index_names = ref_trait.list_index_names_(&mock).unwrap();
            let mut expected_index_names = stored.indexes.keys().cloned().collect::<Vec<_>>();
            expected_index_names.sort();
            assert_eq!(index_names, expected_index_names,);
        }
    }
}
