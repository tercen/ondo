//index.rs
use crate::db::entity::OndoKey;
use serde::{Deserialize, Serialize};

mod key_value;

use crate::db::reference::IndexReference;
pub(crate) use key_value::*;

pub(crate) const DEFAULT_ID_FIELD: &str = "_id";

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub(crate) struct Index {
    pub reference: IndexReference,
    pub fields: Vec<String>,
    //TODO: Return not the end key but the following key to start next time when there is limit
}

pub(crate) type IndexStored = Index;

impl Index {
    pub fn get_fields(&self) -> Vec<String> {
        let mut my_fields = self.fields.clone();
        my_fields.push(DEFAULT_ID_FIELD.to_owned());
        my_fields
    }

    /// Get the index key for a given document.
    ///
    /// This function supports nested properties by splitting field names
    /// containing dots, e.g., "label.property1.property1a", and recursively
    /// navigating through the document structure to retrieve the corresponding
    /// values.
    ///
    /// The extracted values are combined into an `OndoKey` object which is
    /// returned as the index key.
    ///
    /// # Example
    ///
    /// Consider the following JSON document:
    ///
    /// {
    ///     "label": {
    ///         "property1": {
    ///             "property1a": "value1a"
    ///         }
    ///     },
    ///     "property2": "value2"
    /// }
    ///
    /// If the index is defined using the fields "label.property1.property1a" and
    /// "property2", this function will navigate the nested structure to obtain
    /// the values "value1a" and "value2" respectively. These values are combined
    /// into an `OndoKey` object, which is then returned as the index key.
    pub fn key_of(&self, doc: &IndexValue) -> IndexKey {
        let fields = self.get_fields();

        let values: Vec<serde_json::Value> = fields
            .iter()
            .map(|f: &String| {
                let item = get_nested_property(doc, f);
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

pub(self) fn get_nested_property(doc: &IndexValue, field: &str) -> serde_json::Value {
    let mut current_value = doc;
    let field_parts = field.split('.').collect::<Vec<&str>>();

    for field_part in field_parts {
        match current_value.get(field_part) {
            Some(value) => {
                if value.is_object() {
                    current_value = value;
                } else {
                    return value.clone();
                }
            }
            None => return serde_json::Value::Null,
        }
    }

    serde_json::Value::Null
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::reference::DomainReference;
    use crate::db::reference::TableReference;

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
                "city".to_owned(),
                "age".to_owned(),
                DEFAULT_ID_FIELD.to_string(),
            ]
        );
    }

    #[test]
    fn test_key_of() {
        let index = sample_index();
        let doc = sample_document_json();

        let expected_key = OndoKey {
            values: vec![json!("New York"), json!(30), json!(1),],
        };

        assert_eq!(index.key_of(&doc), expected_key);
    }

    #[test]
    fn test_get_nested_property_single_level() {
        let doc = json!({
            "name": "John Doe",
            "age": 30
        });

        let value = get_nested_property(&doc, "name");
        assert_eq!(value, json!("John Doe"));

        let value = get_nested_property(&doc, "age");
        assert_eq!(value, json!(30));
    }

    #[test]
    fn test_get_nested_property_multi_level() {
        let doc = json!({
            "name": "John Doe",
            "age": 30,
            "address": {
                "city": "New York",
                "country": "USA"
            }
        });

        let value = get_nested_property(&doc, "address.city");
        assert_eq!(value, json!("New York"));

        let value = get_nested_property(&doc, "address.country");
        assert_eq!(value, json!("USA"));
    }

    #[test]
    fn test_get_nested_property_nonexistent() {
        let doc = json!({
            "name": "John Doe",
            "age": 30,
            "address": {
                "city": "New York",
                "country": "USA"
            }
        });

        let value = get_nested_property(&doc, "nonexistent");
        assert_eq!(value, serde_json::Value::Null);

        let value = get_nested_property(&doc, "address.nonexistent");
        assert_eq!(value, serde_json::Value::Null);
    }

    #[test]
    fn test_get_nested_property_deeply_nested() {
        let doc = json!({
            "person": {
                "name": "John Doe",
                "age": 30,
                "address": {
                    "city": "New York",
                    "country": "USA"
                }
            }
        });

        let value = get_nested_property(&doc, "person.name");
        assert_eq!(value, json!("John Doe"));

        let value = get_nested_property(&doc, "person.address.city");
        assert_eq!(value, json!("New York"));
    }
}
