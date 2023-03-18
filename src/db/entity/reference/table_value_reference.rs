//table_value_reference.rs
//TODO!XXX: put -> delete-put self index
//TODO!XXX: delete -> delete self index
//TODO!XXX: post -> put self index
//TODO!XXX!XXX!XXX: post -> create id
//TODO!XXX: find by index
use super::requests::table_value_requests::TableValueRequests;
use super::{
    effect::{Effect, Effects},
    requests::column_value_requests::ColumnValueRequests,
    CfNameMaker, TableReference,
};
use crate::db::entity::index::DEFAULT_ID_FIELD;
use crate::db::entity::reference::column_value_reference::ColumnValueReference;
use crate::db::entity::reference::column_value_reference::ColumnValueReferenceTrait;
use crate::db::entity::reference::effect::table_value_effect::TableValueEffect;
use crate::db::{
    db_error::DbResult,
    entity::{IndexValue, TableValue},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub(crate) trait TableValueReferenceTrait {
    fn container_cf_name(&self) -> String;
    fn get_table_value(&self, request: &dyn TableValueRequests) -> DbResult<Option<TableValue>>;
    fn put_table_value(&self, value: &TableValue) -> DbResult<Effects>;
    fn post_table_value(
        &mut self,
        value: &mut TableValue,
        column_value_requests: &dyn ColumnValueRequests,
    ) -> DbResult<(Value, Effects)>;
    fn delete_table_value(&self) -> DbResult<Effects>;
}

pub type TableKey = IndexValue;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub(crate) struct TableValueReference {
    pub table_reference: TableReference,
    pub id: TableKey,
}

impl TableValueReference {
    pub fn new(domain_name: &str, table_name: &str, id: IndexValue) -> Self {
        TableValueReference {
            table_reference: TableReference::new(domain_name, table_name),
            id,
        }
    }

    pub fn to_table_reference(&self) -> TableReference {
        self.table_reference.clone()
    }
}

impl TableValueReferenceTrait for TableValueReference {
    fn container_cf_name(&self) -> String {
        CfNameMaker::for_table_values(&self.table_reference)
    }

    fn get_table_value(&self, request: &dyn TableValueRequests) -> DbResult<Option<TableValue>> {
        request.get_table_value(&self.container_cf_name(), &self)
    }

    fn put_table_value(&self, value: &TableValue) -> DbResult<Effects> {
        let effect = Effect::TableValueEffect(TableValueEffect::Put(
            self.container_cf_name(),
            self.clone(),
            value.clone(),
        ));
        Ok(vec![effect])
    }

    fn post_table_value(
        &mut self,
        value: &mut TableValue,
        column_value_requests: &dyn ColumnValueRequests,
    ) -> DbResult<(Value, Effects)> {
        let mut effects: Vec<Effect> = Vec::new();
        let existing_id_opt = value.get(DEFAULT_ID_FIELD);
        let no_id = serde_json::json!(0u64);
        let id_not_found = existing_id_opt.is_none() || existing_id_opt == Some(&no_id);

        let id_used = match id_not_found {
            true => {
                let domain_reference = self.table_reference.to_domain_reference();
                let table_counter_reference = ColumnValueReference {
                    column_reference: domain_reference.cf_name_for_table_counters(),
                    id: serde_json::json!(self.table_reference.table_name),
                };
                let (new_id, counter_effects) =
                    table_counter_reference.increment_column_value(column_value_requests)?;
                effects.extend(counter_effects);
                if let Some(obj) = value.as_object_mut() {
                    obj.insert(
                        DEFAULT_ID_FIELD.to_owned(),
                        Value::Number(serde_json::Number::from(new_id)),
                    );
                }
                serde_json::json!(new_id)
            }
            false => {
                let value = existing_id_opt.unwrap();
                value.clone()
            }
        };
        self.id = id_used.clone();
        let put_effects = self.put_table_value(value)?;
        effects.extend(put_effects);
        Ok((id_used, effects))
    }

    fn delete_table_value(&self) -> DbResult<Effects> {
        let effect = Effect::TableValueEffect(TableValueEffect::Delete(
            self.container_cf_name(),
            self.clone(),
        ));
        Ok(vec![effect])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::*;
    use serde_json::json;
    use serde_json::value::Number;
    use serde_json::Value;

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

    fn create_table_key() -> TableKey {
        Value::Number(Number::from(1))
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
            let table_value_ref =
                create_table_value_ref("sample_domain", "sample_table", create_table_key());
            let table_value = create_table_value();

            let effects = table_value_ref.put_table_value(&table_value).unwrap();
            let expected_effect = Effect::TableValueEffect(TableValueEffect::Put(
                "sample_domain::/sample_table".to_owned(),
                table_value_ref.clone(),
                table_value,
            ));

            assert_eq!(effects.len(), 1);
            assert_eq!(effects[0], expected_effect);
        }

        #[test]
        fn test_delete_table_value() {
            let table_value_ref =
                create_table_value_ref("sample_domain", "sample_table", create_table_key());
            let expected_effect = Effect::TableValueEffect(TableValueEffect::Delete(
                table_value_ref.container_cf_name(),
                table_value_ref.clone(),
            ));
            let result = table_value_ref.delete_table_value();
            assert_eq!(result, Ok(vec![expected_effect]));
        }
    }
}
