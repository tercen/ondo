//table_value_reference.rs
//TODO!XXX: find by index
use super::effect::{Effect, Effects};
use super::requests::column_value_requests::ColumnValueRequests;
use super::requests::table_value_requests::TableValueRequests;
use super::CfNameMaker;
use crate::db::entity::index::DEFAULT_ID_FIELD;
use crate::db::entity::ondo_key::OndoKey;
use crate::db::entity::ondo_key::OptionalOndoKey;
use crate::db::entity::reference::column_value_reference::ColumnValueReference;
use crate::db::entity::reference::column_value_reference::ColumnValueReferenceTrait;
use crate::db::entity::reference::effect::table_value_effect::TableValueEffect;
use crate::db::entity::reference::requests::table_stored_requests::TableStoredRequests;
use crate::db::entity::reference::table_reference::stored::TableStoredReferenceTrait;
use crate::db::entity::reference::TableReference;
use crate::db::entity::table_value::do_deindex_table_value;
use crate::db::entity::table_value::do_index_table_value;
use crate::db::{db_error::DbResult, entity::TableValue};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub(crate) trait TableValueReferenceTrait {
    fn container_cf_name(&self) -> String;
    fn get_table_value(&self, request: &dyn TableValueRequests) -> DbResult<Option<TableValue>>;
    fn put_table_value(
        &self,
        value: &TableValue,
        table_stored_requests: &dyn TableStoredRequests,
        table_value_requests: &dyn TableValueRequests,
    ) -> DbResult<Effects>;
    fn delete_table_value(
        &self,
        table_stored_requests: &dyn TableStoredRequests,
        table_value_requests: &dyn TableValueRequests,
    ) -> DbResult<Effects>;
}
pub(crate) trait CreateTableValueReferenceTrait {
    fn container_cf_name(&self) -> String;
    fn post_table_value(
        &self,
        value: &mut TableValue,
        column_value_requests: &dyn ColumnValueRequests,
        table_stored_requests: &dyn TableStoredRequests,
        table_value_requests: &dyn TableValueRequests,
    ) -> DbResult<(OndoKey, Effects)>;
}

pub(crate) type TableKey = OndoKey;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub(crate) struct TableValueReference {
    pub table_reference: TableReference,
    pub id: OndoKey,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub(crate) struct CreateTableValueReference {
    pub table_reference: TableReference,
    pub id: OptionalOndoKey,
}

impl TableValueReference {
    pub fn new(domain_name: &str, table_name: &str, id: OndoKey) -> Self {
        TableValueReference {
            table_reference: TableReference::new(domain_name, table_name),
            id,
        }
    }

    pub fn to_table_reference(&self) -> TableReference {
        self.table_reference.clone()
    }
}
impl CreateTableValueReference {
    pub fn to_table_reference(&self) -> TableReference {
        self.table_reference.clone()
    }
}

fn do_indexing(
    table_value_reference: &TableValueReference,
    table_value: &TableValue,
    table_stored_requests: &dyn TableStoredRequests,
) -> DbResult<Effects> {
    let table_reference = table_value_reference.to_table_reference();
    let table_stored = table_reference
        .get_table_stored(table_stored_requests)?
        .ok_or(crate::db::db_error::DbError::TableNotInitialized)?;
    let mut effects: Vec<Effect> = Vec::new();
    for the_index in table_stored.indexes.values() {
        let index_effects = do_index_table_value(table_value, the_index)?;
        effects.extend(index_effects);
    }
    Ok(effects)
}

fn do_deindexing(
    table_value_reference: &TableValueReference,
    table_value: &TableValue,
    table_stored_requests: &dyn TableStoredRequests,
) -> DbResult<Effects> {
    let table_reference = table_value_reference.to_table_reference();
    let table_stored = table_reference
        .get_table_stored(table_stored_requests)?
        .ok_or(crate::db::db_error::DbError::TableNotInitialized)?;
    let mut effects: Vec<Effect> = Vec::new();
    for the_index in table_stored.indexes.values() {
        let index_effects = do_deindex_table_value(table_value, the_index)?;
        effects.extend(index_effects);
    }
    Ok(effects)
}

impl Into<OndoKey> for u64 {
    fn into(self) -> OndoKey {
        let value = Value::Number(self.into());
        let values = vec![value];
        OndoKey { values }
    }
}

impl CreateTableValueReferenceTrait for CreateTableValueReference {
    fn container_cf_name(&self) -> String {
        CfNameMaker::for_table_values(&self.table_reference)
    }

    fn post_table_value(
        &self,
        value: &mut TableValue,
        column_value_requests: &dyn ColumnValueRequests,
        table_stored_requests: &dyn TableStoredRequests,
        _table_value_requests: &dyn TableValueRequests,
    ) -> DbResult<(OndoKey, Effects)> {
        let mut effects: Vec<Effect> = Vec::new();

        let id_used: OndoKey = match self.id.clone() {
            None => {
                let domain_reference = self.table_reference.to_domain_reference();
                let table_counter_reference = ColumnValueReference {
                    column_reference: domain_reference.cf_name_for_table_counters(),
                    id: serde_json::json!(self.table_reference.table_name),
                };
                let (new_id_int, counter_effects) =
                    table_counter_reference.increment_column_value(column_value_requests)?;
                effects.extend(counter_effects);

                let new_ondo_key: OndoKey = new_id_int.into();
                if let Some(obj) = value.as_object_mut() {
                    obj.insert(
                        DEFAULT_ID_FIELD.to_owned(),
                        serde_json::to_value(&new_ondo_key).unwrap(),
                    );
                }
                new_ondo_key
            }
            Some(user_key) => user_key,
        };
        let new_reference = TableValueReference {
            table_reference: self.table_reference.clone(),
            id: id_used.clone(),
        };
        let index_effects = do_indexing(&new_reference, value, table_stored_requests)?;
        effects.extend(index_effects);
        let put_effect = Effect::TableValueEffect(TableValueEffect::Put(
            self.container_cf_name(),
            new_reference.id.clone(),
            value.clone(),
        ));
        effects.push(put_effect);
        Ok((id_used, effects))
    }
}

impl TableValueReferenceTrait for TableValueReference {
    fn container_cf_name(&self) -> String {
        CfNameMaker::for_table_values(&self.table_reference)
    }

    fn get_table_value(&self, request: &dyn TableValueRequests) -> DbResult<Option<TableValue>> {
        request.get_table_value(&self.container_cf_name(), &self)
    }

    fn put_table_value(
        &self,
        value: &TableValue,
        table_stored_requests: &dyn TableStoredRequests,
        table_value_requests: &dyn TableValueRequests,
    ) -> DbResult<Effects> {
        let mut effects = vec![];
        let old_value = self
            .get_table_value(table_value_requests)?
            .ok_or(crate::db::db_error::DbError::NotFound)?;
        let deindex_effects = do_deindexing(self, &old_value, table_stored_requests)?;
        let index_effects = do_indexing(self, value, table_stored_requests)?;
        effects.extend(deindex_effects);
        effects.extend(index_effects);
        let put_effect = Effect::TableValueEffect(TableValueEffect::Put(
            self.container_cf_name(),
            self.id.clone(),
            value.clone(),
        ));
        effects.push(put_effect);
        Ok(effects)
    }

    fn delete_table_value(
        &self,
        table_stored_requests: &dyn TableStoredRequests,
        table_value_requests: &dyn TableValueRequests,
    ) -> DbResult<Effects> {
        let effect = Effect::TableValueEffect(TableValueEffect::Delete(
            self.container_cf_name(),
            self.id.clone(),
        ));
        let mut effects = vec![effect];
        let old_value = self
            .get_table_value(table_value_requests)?
            .ok_or(crate::db::db_error::DbError::NotFound)?;
        let deindex_effects = do_deindexing(self, &old_value, table_stored_requests)?;
        effects.extend(deindex_effects);
        Ok(effects)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::entity::reference::table_reference::stored::tests::{
        create_table_stored, MockTableStoredTestRequests,
    };
    use mockall::*;
    use serde_json::json;

    mock! {
        pub(crate) TableValueTestRequests {}
        impl TableValueRequests for TableValueTestRequests {
            fn get_table_value(
                &self,
                cf_name: &str,
                key: &TableValueReference,
            ) -> DbResult<Option<TableValue>>;
        }
    }

    pub(crate) fn create_table_value_ref(
        domain_name: &str,
        table_name: &str,
        key: TableKey,
    ) -> TableValueReference {
        TableValueReference::new(domain_name, table_name, key)
    }

    pub fn create_table_value() -> TableValue {
        json!({
            "_id": 1,
            "name": "John",
            "surname": "Doe",
            "age": 30
        })
    }

    fn create_table_key() -> OndoKey {
        1u64.into()
    }

    mod table_value_reference_trait_tests {
        use super::*;
        use crate::db::entity::reference::effect::table_value_effect::TableValueEffect;

        #[test]
        fn test_get_table_value() {
            let mut mock = MockTableValueTestRequests::new();
            let table_value_ref =
                create_table_value_ref("sample_domain", "sample_table", create_table_key());
            let expected_table_value = create_table_value();

            mock.expect_get_table_value()
                .returning(move |_, _| Ok(Some(create_table_value())));

            let result = table_value_ref.get_table_value(&mock);

            assert_eq!(result, Ok(Some(expected_table_value)));
        }
        #[test]
        fn test_put_table_value() {
            let mut table_mock = MockTableStoredTestRequests::new();
            table_mock
                .expect_get_table_stored()
                .returning(move |_, _| Ok(Some(create_table_stored())));

            let mut mock = MockTableValueTestRequests::new();
            mock.expect_get_table_value()
                .returning(move |_, _| Ok(Some(create_table_value())));

            let table_value_ref =
                create_table_value_ref("sample_domain", "sample_table", create_table_key());
            let table_value = create_table_value();

            let effects = table_value_ref
                .put_table_value(&table_value, &table_mock, &mock)
                .unwrap();
            let expected_effect = Effect::TableValueEffect(TableValueEffect::Put(
                "sample_domain::/sample_table".to_owned(),
                table_value_ref.id.clone(),
                table_value,
            ));

            assert_eq!(effects.len(), 1);
            assert_eq!(effects[0], expected_effect);
        }

        #[test]
        fn test_delete_table_value() {
            let mut table_mock = MockTableStoredTestRequests::new();
            table_mock
                .expect_get_table_stored()
                .returning(move |_, _| Ok(Some(create_table_stored())));
            let mut mock = MockTableValueTestRequests::new();
            mock.expect_get_table_value()
                .returning(move |_, _| Ok(Some(create_table_value())));

            let table_value_ref =
                create_table_value_ref("sample_domain", "sample_table", create_table_key());
            let expected_effect = Effect::TableValueEffect(TableValueEffect::Delete(
                table_value_ref.container_cf_name(),
                table_value_ref.id.clone(),
            ));
            let result = table_value_ref.delete_table_value(&table_mock, &mock);
            assert_eq!(result, Ok(vec![expected_effect]));
        }
    }
}
//TEST:: Missing test for post_table_value
