//table_value.rs
use crate::db::db_error::DbResult;
use crate::db::entity::index::Index;
use crate::db::entity::reference::effect::Effects;
use crate::db::entity::reference::index_value_reference::IndexValueReference;
use crate::db::entity::reference::index_value_reference::IndexValueReferenceTrait;

pub(crate) type TableValue = serde_json::Value;

pub(crate) fn do_index_table_value(value: &TableValue, the_index: &Index) -> DbResult<Effects> {
    let key_value = the_index.key_value_of(&value);
    let index_value_reference = IndexValueReference {
        index_reference: the_index.id.clone(),
        key: key_value.key,
    };
    let r_index_value_effects = index_value_reference.put_index_value(&key_value.value);
    r_index_value_effects
}

pub(crate) fn do_deindex_table_value(value: &TableValue, the_index: &Index) -> DbResult<Effects> {
    let key_value = the_index.key_value_of(&value);
    let index_value_reference = IndexValueReference {
        index_reference: the_index.id.clone(),
        key: key_value.key,
    };
    let r_index_value_effects = index_value_reference.delete_index_value();
    r_index_value_effects
}
