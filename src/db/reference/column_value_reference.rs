//column_value_reference.rs
use crate::db::entity::index::IndexKey;
use crate::db::entity::table_value::TableValue;
use crate::db::reference::effect::{AccessEffect, Effect, Effects};
use crate::db::{
    entity::IndexValue,
    reference::{effect::ColumnValueEffect, requests::ColumnValueRequests},
    DbError, DbResult,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub(crate) type ColumnKey = IndexKey;
pub(crate) type ColumnValue = TableValue;

pub(crate) trait ColumnValueReferenceTrait {
    fn get_column_value(&self, request: &dyn ColumnValueRequests) -> DbResult<Option<ColumnValue>>;
    fn put_column_value(&self, value: &ColumnValue) -> DbResult<Effects>;
    fn delete_column_value(&self) -> DbResult<Effects>;
    fn increment_column_value(&self, request: &dyn ColumnValueRequests)
        -> DbResult<(u64, Effects)>;
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub(crate) struct ColumnValueReference {
    pub column_reference: String,
    pub id: ColumnKey,
}

impl ColumnValueReference {
    pub fn new(cf_name: &str, id: IndexValue) -> Self {
        ColumnValueReference {
            column_reference: cf_name.to_owned(),
            id,
        }
    }
}

impl ColumnValueReferenceTrait for ColumnValueReference {
    fn get_column_value(&self, request: &dyn ColumnValueRequests) -> DbResult<Option<ColumnValue>> {
        request.get_column_value(&self.column_reference, &self.id)
    }

    fn put_column_value(&self, value: &ColumnValue) -> DbResult<Effects> {
        let effect = Effect::Access(AccessEffect::ColumnValueEffect(ColumnValueEffect::Put(
            self.column_reference.clone(),
            self.id.clone(),
            value.clone(),
        )));
        Ok(vec![effect])
    }

    fn delete_column_value(&self) -> DbResult<Effects> {
        let effect = Effect::Access(AccessEffect::ColumnValueEffect(ColumnValueEffect::Delete(
            self.column_reference.clone(),
            self.id.clone(),
        )));
        Ok(vec![effect])
    }

    fn increment_column_value(
        &self,
        request: &dyn ColumnValueRequests,
    ) -> DbResult<(u64, Effects)> {
        let value = match self.get_column_value(request)? {
            Some(value) => value,
            None => json!(0u64),
        };
        let n = match value.as_u64() {
            Some(number) => number,
            None => return Err(DbError::NotU64),
        };
        let new_n = n + 1;
        let new_value = json!(new_n);
        let effects = self.put_column_value(&new_value)?;
        Ok((new_n, effects))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::entity::ondo_key::OndoKey;
    use mockall::*;
    use serde_json::json;

    mock! {
        pub(crate) ColumnValueTestRequests {}
        impl ColumnValueRequests for ColumnValueTestRequests {
            fn get_column_value(
                &self,
                cf_name: &str,
                key: &ColumnKey,
            ) -> DbResult<Option<ColumnValue>>;
        }
    }

    pub(crate) fn create_column_value_ref(
        column_name: &str,
        key: ColumnKey,
    ) -> ColumnValueReference {
        ColumnValueReference::new(column_name, key)
    }

    pub fn create_column_value() -> ColumnValue {
        json!({
            "_id": 1,
            "name": "John",
            "surname": "Doe",
            "age": 30
        })
    }

    fn create_column_key() -> ColumnKey {
        let ondo_key: OndoKey = 1u64.into();
        ondo_key
    }

    mod column_value_reference_trait_tests {
        use super::*;

        #[test]
        fn test_get_column_value() {
            let mut mock = MockColumnValueTestRequests::new();
            let column_value_ref = create_column_value_ref("sample_column", create_column_key());
            let expected_column_value = create_column_value();

            mock.expect_get_column_value()
                .returning(move |_, _| Ok(Some(create_column_value())));

            let result = column_value_ref.get_column_value(&mock);

            assert_eq!(result, Ok(Some(expected_column_value)));
        }
        #[test]
        fn test_put_column_value() {
            let column_value_ref = create_column_value_ref("sample_column", create_column_key());
            let column_value = create_column_value();

            let effects = column_value_ref.put_column_value(&column_value).unwrap();
            let expected_effect =
                Effect::Access(AccessEffect::ColumnValueEffect(ColumnValueEffect::Put(
                    column_value_ref.column_reference.clone(),
                    column_value_ref.id.clone(),
                    column_value,
                )));

            assert_eq!(effects.len(), 1);
            assert_eq!(effects[0], expected_effect);
        }

        #[test]
        fn test_delete_column_value() {
            let column_value_ref = create_column_value_ref("sample_column", create_column_key());
            let expected_effect =
                Effect::Access(AccessEffect::ColumnValueEffect(ColumnValueEffect::Delete(
                    column_value_ref.column_reference.clone(),
                    column_value_ref.id.clone(),
                )));
            let result = column_value_ref.delete_column_value();
            assert_eq!(result, Ok(vec![expected_effect]));
        }

        #[test]
        fn test_increment_column_value() {
            let mut mock = MockColumnValueTestRequests::new();
            let column_value_ref = create_column_value_ref("sample_column", create_column_key());
            let initial_column_value = json!(1u64);
            let expected_column_value = json!(2u64);

            let expected_effect =
                Effect::Access(AccessEffect::ColumnValueEffect(ColumnValueEffect::Put(
                    column_value_ref.column_reference.clone(),
                    column_value_ref.id.clone(),
                    expected_column_value.clone(),
                )));

            mock.expect_get_column_value()
                .returning(move |_, _| Ok(Some(initial_column_value.clone())));

            let result = column_value_ref.increment_column_value(&mock);
            assert_eq!(result, Ok((2u64, vec![expected_effect])));
        }

        #[test]
        fn test_increment_column_value_initial() {
            let mut mock = MockColumnValueTestRequests::new();
            let column_value_ref = create_column_value_ref("sample_column", create_column_key());
            let expected_column_value = json!(1u64);

            let expected_effect =
                Effect::Access(AccessEffect::ColumnValueEffect(ColumnValueEffect::Put(
                    column_value_ref.column_reference.clone(),
                    column_value_ref.id.clone(),
                    expected_column_value.clone(),
                )));

            mock.expect_get_column_value()
                .returning(move |_, _| Ok(None));

            let result = column_value_ref.increment_column_value(&mock);
            assert_eq!(result, Ok((1u64, vec![expected_effect])));
        }
    }
}
