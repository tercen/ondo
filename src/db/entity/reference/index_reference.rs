//index_reference.rs
//TODO: validate index name
//TODO!XXX!XXX: post-> create index values
//TODO!XXX!XXX: put-> drop cf, create index values
use super::effect::Effect;
use super::effect::Effects;
use super::table_reference::stored::TableStoredReferenceTrait;
use super::table_reference::TableReferenceTrait;
use super::CfNameMaker;
use super::DomainReference;
use super::TableReference;
use crate::db::entity::reference::requests::table_stored_requests::TableStoredRequests;
use crate::db::entity::table_value::do_index_table_value;
use crate::db::{
    db_error::{DbError, DbResult},
    entity::Index,
};
use serde::{Deserialize, Serialize};

pub(crate) trait IndexReferenceTrait {
    fn value_cf_name(&self) -> String;
    fn required_cf_names(&self) -> Vec<String>;
    fn get_index(&self, parent_requests: &dyn TableStoredRequests) -> DbResult<Option<Index>>;
    fn put_index(
        &self,
        index: &Index,
        parent_requests: &dyn TableStoredRequests,
    ) -> DbResult<Effects>;
    fn post_index(
        &self,
        index: &Index,
        parent_requests: &dyn TableStoredRequests,
    ) -> DbResult<Effects>;
    fn delete_index(&self, parent_requests: &dyn TableStoredRequests) -> DbResult<Effects>;
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct IndexReference {
    pub index_name: String,
    pub table_reference: TableReference,
}

impl IndexReference {
    pub fn new(domain_name: &str, table_name: &str, index_name: &str) -> Self {
        IndexReference {
            table_reference: TableReference {
                domain_reference: DomainReference::new(domain_name),
                table_name: table_name.to_string(),
            },

            index_name: index_name.to_string(),
        }
    }

    pub fn to_table_reference(&self) -> TableReference {
        self.table_reference.clone()
    }
}

trait IndexReferencePrivateTrait {
    fn recreate_index_values_cf(&self) -> Effects;
    fn index_related_table_values(
        &self,
        table_requests: &dyn TableStoredRequests,
    ) -> DbResult<Effects>;
    fn create_required_cfs(&self) -> Effects;
    fn delete_required_cfs(&self) -> Effects;
}

impl IndexReferencePrivateTrait for IndexReference {
    fn recreate_index_values_cf(&self) -> Effects {
        let delete_effect = Effect::DeleteCf(self.value_cf_name());
        let create_effect = Effect::CreateCf(self.value_cf_name());
        vec![delete_effect, create_effect]
    }

    fn index_related_table_values(
        &self,
        table_stored_requests: &dyn TableStoredRequests,
    ) -> DbResult<Effects> {
        let table_reference = self.to_table_reference();
        let the_index = self
            .get_index(table_stored_requests)?
            .ok_or(DbError::IndexNotInitialized)?;
        let mut all_values = table_reference.all_values(table_stored_requests);
        let nested_effects = all_values.try_fold(vec![], |mut acc, value| {
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

    fn put_index(
        &self,
        index: &Index,
        parent_requests: &dyn TableStoredRequests,
    ) -> DbResult<Effects> {
        let table_stored_opt = self.table_reference.get_table_stored(parent_requests)?;
        let mut table_stored = table_stored_opt.ok_or(DbError::TableNotInitialized)?;
        let result = table_stored
            .indexes
            .insert(self.index_name.clone(), index.clone());
        if result == None {
            Err(DbError::IndexNotInitialized)
        } else {
            self.recreate_index_values_cf();
            self.table_reference.put_table_stored(&table_stored)
        }
    }

    fn post_index(
        &self,
        index: &Index,
        parent_requests: &dyn TableStoredRequests,
    ) -> DbResult<Effects> {
        let table_stored_opt = self.table_reference.get_table_stored(parent_requests)?;
        let mut table_stored = table_stored_opt.ok_or(DbError::TableNotInitialized)?;
        let result = table_stored
            .indexes
            .insert(self.index_name.clone(), index.clone());
        if result == None {
            let mut effects = self.create_required_cfs();
            let put_effects = self.table_reference.put_table_stored(&table_stored)?;
            effects.extend(put_effects);
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::entity::reference::effect::table_stored_effect::TableStoredEffect;
    use crate::db::entity::reference::table_reference::stored::tests::{
        create_table, create_table_stored, MockTableStoredTestRequests,
    };
    use crate::db::entity::{Table, TableStored};

    fn create_index_ref() -> IndexReference {
        IndexReference::new("sample_domain", "sample_table", "sample_index")
    }

    fn create_index() -> Index {
        Index {
            id: create_index_ref(),
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
                IndexReference::new("sample_domain", "sample_table", "sample_index");
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
                IndexReference::new("sample_domain", "sample_table", "sample_index");

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
            let index_reference =
                IndexReference::new("sample_domain", "sample_table", "sample_index");
            let index = create_index();
            let table_stored = create_table_stored_with_index(&index);
            parent_mock
                .expect_get_table_stored()
                .returning(move |_, _| Ok(Some(table_stored.clone())));

            let effects = index_reference.put_index(&index, &parent_mock);
            assert!(effects.is_ok());
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
                    indexes: [(
                        "sample_index".to_owned(),
                        Index {
                            id: IndexReference {
                                index_name: "sample_index".to_owned(),
                                table_reference: TableReference {
                                    domain_reference: DomainReference::new("sample_domain"),
                                    table_name: "sample_table".to_owned(),
                                },
                            },
                            fields: vec!["sample_field".to_owned()],
                        },
                    )]
                    .into_iter()
                    .collect(),
                },
            ))];
            assert_eq!(effects.unwrap(), expected_effects);
        }

        #[test]
        fn test_put_index_failure() {
            let mut parent_mock = MockTableStoredTestRequests::new();
            let index_reference =
                IndexReference::new("sample_domain", "sample_table", "sample_index");
            let index = create_index();
            let table_stored = create_table_stored();
            parent_mock
                .expect_get_table_stored()
                .returning(move |_, _| Ok(Some(table_stored.clone())));

            let effects = index_reference.put_index(&index, &parent_mock);
            assert!(effects.is_err());

            assert_eq!(effects.unwrap_err(), DbError::IndexNotInitialized);
        }
        #[test]
        fn test_post_index() {
            let mut parent_mock = MockTableStoredTestRequests::new();
            let index_reference =
                IndexReference::new("sample_domain", "sample_table", "sample_index");
            let index = create_index();
            let table_stored = create_table_stored();
            parent_mock
                .expect_get_table_stored()
                .returning(move |_, _| Ok(Some(table_stored.clone())));

            let effects = index_reference.post_index(&index, &parent_mock);
            assert!(effects.is_ok());
            let expected_effects = vec![
                Effect::CreateCf("sample_domain::/sample_table/indexes/sample_index".to_owned()),
                Effect::TableStoredEffect(TableStoredEffect::Put(
                    "/domains/sample_domain/tables".to_owned(),
                    "sample_table".to_owned(),
                    TableStored {
                        table: Table {
                            reference: TableReference {
                                domain_reference: DomainReference::new("sample_domain"),
                                table_name: "sample_table".to_owned(),
                            },
                        },
                        indexes: [(
                            "sample_index".to_owned(),
                            Index {
                                id: IndexReference {
                                    index_name: "sample_index".to_owned(),
                                    table_reference: TableReference {
                                        domain_reference: DomainReference::new("sample_domain"),
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
            let index_reference =
                IndexReference::new("sample_domain", "sample_table", "sample_index");
            let index = create_index();
            let table_stored = create_table_stored_with_index(&index);
            parent_mock
                .expect_get_table_stored()
                .returning(move |_, _| Ok(Some(table_stored.clone())));

            let effects = index_reference.post_index(&index, &parent_mock);
            assert!(effects.is_err());

            assert_eq!(effects.unwrap_err(), DbError::AlreadyExists);
        }

        fn test_delete_index() {
            let mut parent_mock = MockTableStoredTestRequests::new();
            let index_reference =
                IndexReference::new("sample_domain", "sample_table", "sample_index");
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
