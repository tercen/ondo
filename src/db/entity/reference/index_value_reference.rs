//index_value_reference.rs
use super::{
    effect::{Effect, Effects},
    CfNameMaker, IndexReference,
};
use crate::db::entity::reference::effect::index_value_effect::IndexValueEffect;
use crate::db::entity::reference::requests::index_value_requests::IndexValueRequests;
use crate::db::{
    db_error::DbResult,
    entity::{IndexKey, IndexValue},
};
use serde::{Deserialize, Serialize};

pub(crate) trait IndexValueReferenceTrait {
    fn container_cf_name(&self) -> String;
    fn get_index_value(&self, requests: &dyn IndexValueRequests) -> DbResult<Option<IndexValue>>;
    fn put_index_value(&self, id: &IndexValue) -> DbResult<Effects>;
    fn delete_index_value(&self) -> DbResult<Effects>;
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub(crate) struct IndexValueReference {
    pub index_reference: IndexReference,
    pub key: IndexKey,
}

impl IndexValueReference {
    pub fn new(domain_name: &str, table_name: &str, index_name: &str, key: IndexKey) -> Self {
        IndexValueReference {
            index_reference: IndexReference::new(domain_name, table_name, index_name),
            key,
        }
    }

    pub fn to_index_reference(&self) -> IndexReference {
        self.index_reference.clone()
    }
}

impl IndexValueReferenceTrait for IndexValueReference {
    fn container_cf_name(&self) -> String {
        CfNameMaker::for_index_values(&self.index_reference)
    }
    fn get_index_value(&self, requests: &dyn IndexValueRequests) -> DbResult<Option<IndexValue>> {
        requests.get_index_value_stored(&self.container_cf_name(), &self)
    }
    fn put_index_value(&self, id: &IndexValue) -> DbResult<Effects> {
        let effect = Effect::IndexValueEffect(IndexValueEffect::Put(
            self.container_cf_name(),
            self.clone(),
            id.clone(),
        ));
        Ok(vec![effect])
    }
    fn delete_index_value(&self) -> DbResult<Effects> {
        let effect = Effect::IndexValueEffect(IndexValueEffect::Delete(
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
    use serde_json::value::Number;
    use serde_json::Value;

    mock! {
        IndexValueTestRequests {}
        impl IndexValueRequests for IndexValueTestRequests {
            fn get_index_value_stored(
                &self,
                cf_name: &str,
                key: &IndexValueReference,
            ) -> DbResult<Option<IndexValue>>;        }
    }

    fn create_index_value_ref(
        domain_name: &str,
        table_name: &str,
        index_name: &str,
        key: IndexKey,
    ) -> IndexValueReference {
        IndexValueReference::new(domain_name, table_name, index_name, key)
    }

    fn create_index_value() -> IndexValue {
        Value::Number(Number::from(1))
    }

    fn create_index_key() -> IndexKey {
        vec![
            Value::String("key1".to_owned()),
            Value::String("key2".to_owned()),
        ]
    }

    mod index_value_reference_trait_tests {
        use super::*;

        #[test]
        fn test_get_index_value() {
            let mut mock = MockIndexValueTestRequests::new();
            let index_value_ref = create_index_value_ref(
                "sample_domain",
                "sample_table",
                "sample_index",
                create_index_key(),
            );
            let expected_index_value = create_index_value();

            mock.expect_get_index_value_stored()
                .returning(move |_, _| Ok(Some(create_index_value())));

            let result = index_value_ref.get_index_value(&mock);

            assert_eq!(result, Ok(Some(expected_index_value)));
        }
        #[test]
        fn test_put_index_value() {
            let index_value_ref = create_index_value_ref(
                "sample_domain",
                "sample_table",
                "sample_index",
                create_index_key(),
            );
            let index_value = create_index_value();

            let effects = index_value_ref.put_index_value(&index_value).unwrap();
            let expected_effect = Effect::IndexValueEffect(IndexValueEffect::Put(
                "sample_domain::/sample_table/indexes/sample_index".to_owned(),
                index_value_ref.clone(),
                index_value,
            ));

            assert_eq!(effects.len(), 1);
            assert_eq!(effects[0], expected_effect);
        }
        #[test]
        fn test_delete_index_value() {
            let index_value_ref = create_index_value_ref(
                "sample_domain",
                "sample_table",
                "sample_index",
                create_index_key(),
            );

            let effects = index_value_ref.delete_index_value().unwrap();
            let expected_effect = Effect::IndexValueEffect(IndexValueEffect::Delete(
                "sample_domain::/sample_table/indexes/sample_index".to_owned(),
                index_value_ref.clone(),
            ));

            assert_eq!(effects.len(), 1);
            assert_eq!(effects[0], expected_effect);
        }
    }
}
