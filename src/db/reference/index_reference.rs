//index_reference.rs
//TODO: validate index name
use crate::db::entity::table_value::do_index_table_value;
use crate::db::enums::TableStoredIteratorRequestsFactoryEnum;
use crate::db::{
    entity::{Index, OndoKey, TableValue},
    reference::{
        requests::{IndexIteratorRequests, TableStoredRequests, TableValueRequests},
        table_reference::stored::TableStoredReferenceTrait,
        CfNameMaker, DomainReference, Effect, Effects, TableReference, TableReferenceTrait,
        TableValueReference, TableValueReferenceTrait,
    },
    DbError, DbResult,
};
use serde::{Deserialize, Serialize};

pub(crate) trait IndexReferenceTrait {
    fn value_cf_name(&self) -> String;
    fn required_cf_names(&self) -> Vec<String>;
    fn get_index(&self, parent_requests: &dyn TableStoredRequests) -> DbResult<Option<Index>>;
    fn put_index<'a>(
        &self,
        index: &Index,
        parent_requests: &dyn TableStoredRequests,
        table_stored_iterator_requests_factory: &TableStoredIteratorRequestsFactoryEnum,
    ) -> DbResult<Effects>;
    fn post_index<'a>(
        &self,
        index: &Index,
        parent_requests: &dyn TableStoredRequests,
        table_stored_iterator_requests_factory: &TableStoredIteratorRequestsFactoryEnum,
    ) -> DbResult<Effects>;
    fn delete_index(&self, parent_requests: &dyn TableStoredRequests) -> DbResult<Effects>;

    fn all_values_with_key_prefix<'a>(
        &self,
        key_prefix: OndoKey,
        table_value_requests: &'a dyn TableValueRequests,
        requests: &'a dyn IndexIteratorRequests<'a>,
    ) -> DbResult<Box<dyn Iterator<Item = DbResult<TableValue>> + 'a>>;
    fn all_index_values_with_key_prefix<'a>(
        &self,
        key_prefix: OndoKey,
        requests: &'a dyn IndexIteratorRequests<'a>,
    ) -> DbResult<Box<dyn Iterator<Item = DbResult<OndoKey>> + 'a>>;
    fn all_values_with_key_range<'a>(
        &self,
        start_key_prefix: OndoKey,
        end_key_prefix: OndoKey,
        table_value_requests: &'a dyn TableValueRequests,
        requests: &'a dyn IndexIteratorRequests<'a>,
    ) -> DbResult<Box<dyn Iterator<Item = DbResult<TableValue>> + 'a>>;
    fn all_index_values_with_key_range<'a>(
        &self,
        start_key_prefix: OndoKey,
        end_key_prefix: OndoKey,
        requests: &'a dyn IndexIteratorRequests<'a>,
    ) -> DbResult<Box<dyn Iterator<Item = DbResult<OndoKey>> + 'a>>;
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct IndexReference {
    pub table_reference: TableReference,
    pub index_name: String,
}

impl IndexReference {
    pub fn build(domain_name: &str, table_name: &str, index_name: &str) -> Self {
        IndexReference {
            table_reference: TableReference {
                domain_reference: DomainReference::build(domain_name),
                table_name: table_name.to_owned(),
            },

            index_name: index_name.to_owned(),
        }
    }
    pub fn new(table_reference: TableReference, index_name: &str) -> Self {
        IndexReference {
            table_reference,
            index_name: index_name.to_owned(),
        }
    }

    pub fn to_table_reference(&self) -> TableReference {
        self.table_reference.clone()
    }
}

trait IndexReferencePrivateTrait<'a> {
    fn recreate_index_values_cf(&self) -> Effects;
    fn index_related_table_values(
        &self,
        the_index: &Index,
        table_stored_iterator_requests_factory: &TableStoredIteratorRequestsFactoryEnum,
    ) -> DbResult<Effects>;
    fn create_required_cfs(&self) -> Effects;
    fn delete_required_cfs(&self) -> Effects;
}

impl<'a> IndexReferencePrivateTrait<'a> for IndexReference {
    fn recreate_index_values_cf(&self) -> Effects {
        let delete_effect = Effect::DeleteCf(self.value_cf_name());
        let create_effect = Effect::CreateCf(self.value_cf_name());
        vec![delete_effect, create_effect]
    }

    fn index_related_table_values(
        &self,
        the_index: &Index,
        table_stored_iterator_requests_factory: &TableStoredIteratorRequestsFactoryEnum,
    ) -> DbResult<Effects> {
        let table_stored_iterator_requests_enum =
            table_stored_iterator_requests_factory.create_read_locked_requests()?;
        let table_stored_iterator_requests = table_stored_iterator_requests_enum.as_trait();
        {
            let table_reference = self.to_table_reference();
            let all_values = table_reference.all_values(table_stored_iterator_requests);
            let nested_effects = all_values?.try_fold(vec![], |mut acc, r_value| {
                let value = r_value?;
                let r_index_value_effects = do_index_table_value(&value, &the_index);
                match r_index_value_effects {
                    Ok(index_value_effect) => {
                        acc.push(index_value_effect);
                        Ok(acc)
                    }
                    Err(e) => Err(e),
                }
            })?;
            let effects = nested_effects.into_iter().flatten().collect::<Vec<_>>();
            Ok(effects)
        }
    }

    fn create_required_cfs(&self) -> Effects {
        let effects = self
            .required_cf_names()
            .iter()
            .map(|cf_name| Effect::CreateCf(cf_name.clone()))
            .collect::<Vec<_>>();
        effects
    }

    fn delete_required_cfs(&self) -> Effects {
        let effects = self
            .required_cf_names()
            .iter()
            .map(|cf_name| Effect::DeleteCf(cf_name.clone()))
            .collect::<Vec<_>>();
        effects
    }
}

impl IndexReferenceTrait for IndexReference {
    fn value_cf_name(&self) -> String {
        CfNameMaker::for_index_values(&self)
    }

    fn required_cf_names(&self) -> Vec<String> {
        vec![self.value_cf_name()]
    }

    fn get_index(&self, parent_requests: &dyn TableStoredRequests) -> DbResult<Option<Index>> {
        let table_stored_opt = self.table_reference.get_table_stored(parent_requests)?;
        let table_stored = table_stored_opt.ok_or(DbError::TableNotInitialized)?;
        Ok(table_stored.indexes.get(&self.index_name).cloned())
    }

    fn put_index<'a>(
        &self,
        index: &Index,
        parent_requests: &dyn TableStoredRequests,
        table_stored_iterator_requests_factory: &TableStoredIteratorRequestsFactoryEnum,
    ) -> DbResult<Effects> {
        let table_stored_opt = self.table_reference.get_table_stored(parent_requests)?;
        let mut table_stored = table_stored_opt.ok_or(DbError::TableNotInitialized)?;
        let result = table_stored
            .indexes
            .insert(self.index_name.clone(), index.clone());
        if result == None {
            Err(DbError::IndexNotInitialized)
        } else {
            let mut effects: Vec<Effect> = Vec::new();
            effects.extend(self.table_reference.put_table_stored(&table_stored)?);
            effects.extend(self.recreate_index_values_cf());
            let index_related_table_values_effects =
                self.index_related_table_values(index, table_stored_iterator_requests_factory)?;
            effects.extend(index_related_table_values_effects);
            Ok(effects)
        }
    }

    fn post_index<'a>(
        &self,
        index: &Index,
        parent_requests: &dyn TableStoredRequests,
        table_stored_iterator_requests_factory: &TableStoredIteratorRequestsFactoryEnum,
    ) -> DbResult<Effects> {
        let table_stored_opt = self.table_reference.get_table_stored(parent_requests)?;
        let mut table_stored = table_stored_opt.ok_or(DbError::TableNotInitialized)?;
        let result = table_stored
            .indexes
            .insert(self.index_name.clone(), index.clone());
        if result == None {
            // new index
            let mut effects = self.create_required_cfs();
            let put_effects = self.table_reference.put_table_stored(&table_stored)?;
            effects.extend(put_effects);
            let index_related_table_values_effects =
                self.index_related_table_values(index, table_stored_iterator_requests_factory)?;
            effects.extend(index_related_table_values_effects);
            Ok(effects)
        } else {
            Err(DbError::AlreadyExists)
        }
    }

    fn delete_index(&self, parent_requests: &dyn TableStoredRequests) -> DbResult<Effects> {
        let table_stored_opt = self.table_reference.get_table_stored(parent_requests)?;
        let mut table_stored = table_stored_opt.ok_or(DbError::TableNotInitialized)?;
        table_stored.indexes.remove(&self.index_name);
        let mut effects = self.table_reference.put_table_stored(&table_stored)?;
        effects.extend(self.delete_required_cfs());
        Ok(effects)
    }

    fn all_values_with_key_prefix<'a>(
        &self,
        key_prefix: OndoKey,
        table_value_requests: &'a dyn TableValueRequests,
        requests: &'a dyn IndexIteratorRequests<'a>,
    ) -> DbResult<Box<dyn Iterator<Item = DbResult<TableValue>> + 'a>> {
        let index_value_iterator = self.all_index_values_with_key_prefix(key_prefix, requests)?;
        let table_reference = self.table_reference.clone();
        let table_value_iterator = index_value_iterator.map(move |ondo_key_result| {
            let ondo_key = ondo_key_result?;
            let table_value_reference = TableValueReference {
                table_reference: table_reference.clone(),
                id: ondo_key,
            };
            table_value_reference
                .get_table_value(table_value_requests)
                .and_then(|opt| opt.ok_or(DbError::NotFound))
        });
        Ok(Box::new(table_value_iterator))
    }

    fn all_index_values_with_key_prefix<'a>(
        &self,
        key_prefix: OndoKey,
        requests: &'a dyn IndexIteratorRequests<'a>,
    ) -> DbResult<Box<dyn Iterator<Item = DbResult<OndoKey>> + 'a>> {
        let index_iterator =
            requests.all_values_with_key_prefix(&self.value_cf_name(), key_prefix)?;
        let index_value_iterator = index_iterator.map(move |index_value_result| {
            let ondo_key = index_value_result?;
            Ok(ondo_key)
        });
        Ok(Box::new(index_value_iterator))
    }

    fn all_values_with_key_range<'a>(
        &self,
        start_key_prefix: OndoKey,
        end_key_prefix: OndoKey,
        table_value_requests: &'a dyn TableValueRequests,
        requests: &'a dyn IndexIteratorRequests<'a>,
    ) -> DbResult<Box<dyn Iterator<Item = DbResult<TableValue>> + 'a>> {
        let index_value_iterator =
            self.all_index_values_with_key_range(start_key_prefix, end_key_prefix, requests)?;
        let table_reference = self.table_reference.clone();
        let table_value_iterator = index_value_iterator.map(move |ondo_key_result| {
            let ondo_key = ondo_key_result?;
            let table_value_reference = TableValueReference {
                table_reference: table_reference.clone(),
                id: ondo_key,
            };
            table_value_reference
                .get_table_value(table_value_requests)
                .and_then(|opt| opt.ok_or(DbError::NotFound))
        });
        Ok(Box::new(table_value_iterator))
    }

    fn all_index_values_with_key_range<'a>(
        &self,
        start_key_prefix: OndoKey,
        end_key_prefix: OndoKey,
        requests: &'a dyn IndexIteratorRequests<'a>,
    ) -> DbResult<Box<dyn Iterator<Item = DbResult<OndoKey>> + 'a>> {
        let index_iterator = requests.all_values_with_key_range(
            &self.value_cf_name(),
            start_key_prefix,
            end_key_prefix,
        )?;
        let index_value_iterator = index_iterator.map(move |index_value_result| {
            let ondo_key = index_value_result?;
            Ok(ondo_key)
        });
        Ok(Box::new(index_value_iterator))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::entity::{Table, TableStored};
    use crate::db::reference::effect::table_stored_effect::TableStoredEffect;
    use crate::db::reference::table_reference::stored::tests::{
        create_table, create_table_stored, MockTableStoredTestRequests,
    };

    fn create_index_ref() -> IndexReference {
        IndexReference::build("sample_domain", "sample_table", "sample_index")
    }

    fn create_index() -> Index {
        Index {
            reference: create_index_ref(),
            fields: vec!["sample_field".to_owned()],
        }
    }

    fn create_table_stored_with_index(index: &Index) -> TableStored {
        TableStored {
            table: create_table(),
            indexes: vec![("sample_index".to_owned(), index.clone())]
                .into_iter()
                .collect(),
        }
    }

    mod index_reference_trait_tests {
        use super::*;
        #[test]
        fn test_get_index() {
            let mut mock = MockTableStoredTestRequests::new();
            let index_reference =
                IndexReference::build("sample_domain", "sample_table", "sample_index");
            let index = create_index();
            let index_clone = index.clone();

            mock.expect_get_table_stored()
                .returning(move |_, _| Ok(Some(create_table_stored_with_index(&index_clone))));

            let result = index_reference.get_index(&mock);
            assert!(result.is_ok());
            let result_unwrapped = result.unwrap();
            assert_eq!(result_unwrapped, Some(index));
        }
        #[test]
        fn test_get_index_failure() {
            let mut mock = MockTableStoredTestRequests::new();
            let index_reference =
                IndexReference::build("sample_domain", "sample_table", "sample_index");

            mock.expect_get_table_stored()
                .returning(move |_, _| Ok(Some(create_table_stored())));

            let result = index_reference.get_index(&mock);
            assert!(result.is_ok());
            let result_unwrapped = result.unwrap();
            assert_eq!(result_unwrapped, None);
        }
        #[test]
        fn test_put_index() {
            let mut parent_mock = MockTableStoredTestRequests::new();
            let iterator_mock_factory = TableStoredIteratorRequestsFactoryEnum::new_mock();
            let index_reference =
                IndexReference::build("sample_domain", "sample_table", "sample_index");
            let index = create_index();
            let table_stored = create_table_stored_with_index(&index);
            parent_mock
                .expect_get_table_stored()
                .returning(move |_, _| Ok(Some(table_stored.clone())));

            let effects = index_reference.put_index(&index, &parent_mock, &iterator_mock_factory);
            let expected_effects = vec![
                Effect::TableStoredEffect(TableStoredEffect::Put(
                    "/domains/sample_domain/tables".to_owned(),
                    "sample_table".to_owned(),
                    TableStored {
                        table: Table {
                            reference: TableReference {
                                domain_reference: DomainReference::build("sample_domain"),
                                table_name: "sample_table".to_owned(),
                            },
                        },
                        indexes: [(
                            "sample_index".to_owned(),
                            Index {
                                reference: IndexReference {
                                    index_name: "sample_index".to_owned(),
                                    table_reference: TableReference {
                                        domain_reference: DomainReference::build("sample_domain"),
                                        table_name: "sample_table".to_owned(),
                                    },
                                },
                                fields: vec!["sample_field".to_owned()],
                            },
                        )]
                        .into_iter()
                        .collect(),
                    },
                )),
                Effect::DeleteCf("sample_domain::/sample_table/indexes/sample_index".to_owned()),
                Effect::CreateCf("sample_domain::/sample_table/indexes/sample_index".to_owned()),
            ];
            assert_eq!(effects.unwrap(), expected_effects);
        }

        #[test]
        fn test_put_index_failure() {
            let mut parent_mock = MockTableStoredTestRequests::new();
            let iterator_mock_factory = TableStoredIteratorRequestsFactoryEnum::new_mock();
            let index_reference =
                IndexReference::build("sample_domain", "sample_table", "sample_index");
            let index = create_index();
            let table_stored = create_table_stored();
            parent_mock
                .expect_get_table_stored()
                .returning(move |_, _| Ok(Some(table_stored.clone())));

            let effects = index_reference.put_index(&index, &parent_mock, &iterator_mock_factory);
            assert!(effects.is_err());

            assert_eq!(effects.unwrap_err(), DbError::IndexNotInitialized);
        }
        #[test]
        fn test_post_index() {
            let mut parent_mock = MockTableStoredTestRequests::new();
            let iterator_mock_factory = TableStoredIteratorRequestsFactoryEnum::new_mock();

            let index_reference =
                IndexReference::build("sample_domain", "sample_table", "sample_index");
            let index = create_index();
            let table_stored = create_table_stored();
            let mut expected_table_stored = table_stored.clone();
            expected_table_stored
                .indexes
                .insert(index_reference.index_name.clone(), index.clone());
            parent_mock
                .expect_get_table_stored()
                .times(1) // First call
                .returning(move |_, _| Ok(Some(table_stored.clone())));
            // parent_mock
            //     .expect_get_table_stored()
            //     .times(1) // Second call
            //     .returning(move |_, _| Ok(Some(expected_table_stored.clone())));

            let effects = index_reference.post_index(&index, &parent_mock, &iterator_mock_factory);
            // assert!(effects.is_ok());
            let expected_effects = vec![
                Effect::CreateCf("sample_domain::/sample_table/indexes/sample_index".to_owned()),
                Effect::TableStoredEffect(TableStoredEffect::Put(
                    "/domains/sample_domain/tables".to_owned(),
                    "sample_table".to_owned(),
                    TableStored {
                        table: Table {
                            reference: TableReference {
                                domain_reference: DomainReference::build("sample_domain"),
                                table_name: "sample_table".to_owned(),
                            },
                        },
                        indexes: [(
                            "sample_index".to_owned(),
                            Index {
                                reference: IndexReference {
                                    index_name: "sample_index".to_owned(),
                                    table_reference: TableReference {
                                        domain_reference: DomainReference::build("sample_domain"),
                                        table_name: "sample_table".to_owned(),
                                    },
                                },
                                fields: vec!["sample_field".to_owned()],
                            },
                        )]
                        .into_iter()
                        .collect(),
                    },
                )),
            ];
            assert_eq!(effects.unwrap(), expected_effects);
        }

        #[test]
        fn test_post_index_failure() {
            let mut parent_mock = MockTableStoredTestRequests::new();
            let iterator_mock_factory = TableStoredIteratorRequestsFactoryEnum::new_mock();
            let index_reference =
                IndexReference::build("sample_domain", "sample_table", "sample_index");
            let index = create_index();
            let table_stored = create_table_stored_with_index(&index);
            parent_mock
                .expect_get_table_stored()
                .returning(move |_, _| Ok(Some(table_stored.clone())));

            let effects = index_reference.post_index(&index, &parent_mock, &iterator_mock_factory);
            assert!(effects.is_err());

            assert_eq!(effects.unwrap_err(), DbError::AlreadyExists);
        }

        fn test_delete_index() {
            let mut parent_mock = MockTableStoredTestRequests::new();
            let index_reference =
                IndexReference::build("sample_domain", "sample_table", "sample_index");
            let index = create_index();
            let table_stored = create_table_stored_with_index(&index);
            parent_mock
                .expect_get_table_stored()
                .returning(move |_, _| Ok(Some(table_stored.clone())));

            let effects = index_reference.delete_index(&parent_mock);
            assert!(effects.is_ok());
            let expected_effects = vec![];
            assert_eq!(effects.unwrap(), expected_effects);
        }
    }
}
