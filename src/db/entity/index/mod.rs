//index.rs
use crate::db::entity::ondo_key::OndoKey;
use serde::{Deserialize, Serialize};

mod key_value;

use super::reference::IndexReference;
pub(crate) use key_value::*;

pub(crate) const DEFAULT_ID_FIELD: &str = "_id";

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub(crate) struct Index {
    pub reference: IndexReference,
    pub fields: Vec<String>,
}

pub(crate) type IndexStored = Index;

impl Index {
    pub fn get_fields(&self) -> Vec<String> {
        let mut my_fields = vec![DEFAULT_ID_FIELD.to_string()];
        my_fields.extend(self.fields.clone());
        my_fields
    }

    /// Get the index key for a given document
    pub fn key_of(&self, doc: &IndexValue) -> IndexKey {
        let fields = self.get_fields();

        let values: Vec<serde_json::Value> = fields
            .iter()
            .map(|f: &String| {
                let item = doc[f].clone();
                item
            })
            .collect();
        OndoKey { values }
    }

    pub(crate) fn key_value_of(&self, doc: &IndexValue) -> KeyValue {
        let key = self.key_of(doc);
        let value = doc[DEFAULT_ID_FIELD].clone();
        KeyValue::new(key, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::entity::reference::DomainReference;
    use crate::db::entity::reference::TableReference;

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
            name: "John".to_owned(),
            age: 30,
            city: "New York".to_owned(),
        };
    }

    fn sample_document_json() -> IndexValue {
        return json!(&sample_document());
    }

    fn sample_index() -> Index {
        Index {
            reference: IndexReference {
                table_reference: TableReference {
                    domain_reference: DomainReference::new("sample_domain"),
                    table_name: "sample_table".to_owned(),
                },
                index_name: "sample_index".to_owned(),
            },
            fields: vec!["city".to_owned(), "age".to_owned()],
        }
    }

    #[test]
    fn test_get_fields() {
        let index = sample_index();
        assert_eq!(
            *index.get_fields(),
            vec![
                DEFAULT_ID_FIELD.to_string(),
                "city".to_owned(),
                "age".to_owned(),
            ]
        );
    }

    #[test]
    fn test_key_of() {
        let index = sample_index();
        let doc = sample_document_json();

        let expected_key = OndoKey {
            values: vec![json!(1), json!("New York"), json!(30)],
        };

        assert_eq!(index.key_of(&doc), expected_key);
    }
}
