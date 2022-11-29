#![allow(dead_code)]
const DEFAULT_ID_FIELD: &str = "_id";

struct IndexDefinition {
    name: String,
    table_name: String,
    fields: Vec<String>,
}

impl IndexDefinition {

    pub fn cf_name(&self) -> String {
        format!("{}::{}", self.table_name, self.name)
    }

    pub fn get_fields(&self) -> Vec<String> {
        let mut my_fields = self.fields.clone();
        my_fields.push(DEFAULT_ID_FIELD.to_string());
        my_fields
    }

    pub fn key_of(&self, doc: &serde_json::Value) -> Vec<serde_json::Value>  {
        let fields: &Vec<String>  = &self.fields;
        fields.iter().map(|f: &String| {
            let item = doc[f].clone();
            item
        }).collect() 
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use serde::{Deserialize, Serialize};
    
    #[derive(Serialize, Deserialize)]
    struct SampleDocument {
        _id: String,
        name: String,
        age: u32,
        city: String,
    }

    fn sample_document() -> SampleDocument {
        return SampleDocument {
            _id: "1".to_string(),
            name: "John".to_string(),
            age: 30,
            city: "New York".to_string(),
        }
    }

    fn sample_document_json() -> serde_json::Value {
        return serde_json::json!(&sample_document());
    }

    fn sample_index()->IndexDefinition {
        return IndexDefinition {
            name: "sample_index".to_string(),
            table_name: "sample_table".to_string(),
            fields: vec!["city".to_string(), "age".to_string()],
        };
    }

    #[test]
    fn test_cf_name() {
        let index = sample_index();
        assert_eq!(index.cf_name(), "sample_table::sample_index");
    }

    #[test]
    fn test_get_fields() {
        let index = sample_index();
        assert_eq!(index.get_fields(), vec!["city".to_string(), "age".to_string(), DEFAULT_ID_FIELD.to_string()]);
    }
}