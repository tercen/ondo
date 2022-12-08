#![allow(dead_code)]
use crate::names::*;
use crate::cf_name::*;
use key_value::*;

pub mod key_value;

pub(crate) const DEFAULT_ID_FIELD: &str = "_id";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexDefinition {
    domain_name: DomainName,
    table_name: TableName,
    name: IndexName,
    fields: Vec<String>,
}

struct PartialIndexDefinition {
    name: String,
    fields: Vec<String>,
}

impl IndexDefinition {
    /// Returns the get cf name of this [`IndexDefinition`].
    pub fn get_cf_name(&self) -> CfName {
        CfName::for_index(
            &self.domain_name,
            &self.table_name,
            &self.name,
        )
    }

    pub fn get_fields(&self) -> Vec<String> {
        let mut my_fields = self.fields.clone();
        my_fields.push(DEFAULT_ID_FIELD.to_string());
        my_fields
    }

    /// Get the index key for a given document
    pub fn key_of(&self, doc: &Value) -> Key {
        let fields  = self.get_fields();
        
            fields
                .iter()
                .map(|f: &String| {
                    let item = doc[f].clone();
                    item
                })
                .collect()
        
    }

    pub fn key_value_of(&self, doc: &Value) -> KeyValue {
        let key = self.key_of(doc);
        let value = doc[DEFAULT_ID_FIELD].clone();
        KeyValue::new(key, value)
    }

    // TODO: add a method to delete an index of a document
    // TODO: add a method to update an index of a document
}

#[cfg(test)]
mod tests {
    use super::*;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[derive(Serialize, Deserialize)]
    struct SampleDocument {
        _id: u64,
        name: String,
        age: u32,
        city: String,
    }

    fn sample_document() -> SampleDocument {
        return SampleDocument {
            _id: 1,
            name: "John".to_string(),
            age: 30,
            city: "New York".to_string(),
        };
    }

    fn sample_document_json() -> Value {
        return json!(&sample_document());
    }

    fn sample_index() -> IndexDefinition {
        IndexDefinition {
            domain_name: DomainName::new("sample_domain"),
            table_name: TableName::new("sample_table"),
            name: IndexName::new("sample_index"),
            fields: vec!["city".to_string(), "age".to_string()],
        }
    }

    #[test]
    fn test_get_cf_name() {
        let index = sample_index();
        assert_eq!(index.get_cf_name().to_string(), "sample_domain::/sample_table/indexes/sample_index");
    }

    #[test]
    fn test_get_fields() {
        let index = sample_index();
        assert_eq!(
            *index.get_fields(),
            vec![
                "city".to_string(),
                "age".to_string(),
                DEFAULT_ID_FIELD.to_string()
            ]
        );
    }

    #[test]
    fn test_key_of() {
        let index = sample_index();
        let doc = sample_document_json();
        assert_eq!(*index.key_of(&doc), vec![json!("New York"), json!(30), json!(1)]);
    }
}
