//table_value.rs
use crate::db::entity::index::DEFAULT_ID_FIELD;
use crate::db::entity::OndoKey;

pub(crate) type TableValue = serde_json::Value;

pub(crate) fn insert_key_into_table_value(value: &mut TableValue, new_ondo_key: &OndoKey) {
    if let Some(obj) = value.as_object_mut() {
        obj.insert(
            DEFAULT_ID_FIELD.to_owned(),
            serde_json::to_value(new_ondo_key).unwrap(),
        );
    }
}

pub(crate) fn get_key_from_table_value(value: &TableValue) -> OndoKey {
    let id_value = value[DEFAULT_ID_FIELD].clone();
    serde_json::from_value(id_value).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ondo_key_operations() {
        // 1. Cast a u64 into OndoKey
        let num: u64 = 42;
        let ondo_key: OndoKey = num.into();

        // 2. Insert OndoKey into TableValue
        let mut table_value = serde_json::json!({}); // Create an empty JSON object
        insert_key_into_table_value(&mut table_value, &ondo_key);

        // 3. Get OndoKey from TableValue
        let retrieved_ondo_key = get_key_from_table_value(&table_value);

        // 4. Assert that the OndoKey is the same as the one that was inserted
        assert_eq!(ondo_key, retrieved_ondo_key, "OndoKeys are not the same");
    }
}
