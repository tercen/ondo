// text_index/create_tantivy_index.rs
use super::get_tantivy_index_path::get_tantivy_index_path;
use super::TextIndex;
use crate::db::entity::DEFAULT_ID_FIELD;
use crate::db::server::lockable_db::LockableDb;
use std::path::Path;
use tantivy::{schema::*, Result as TantivyResult};

pub(super) fn create_tantivy_index(
    text_index: &TextIndex,
    lockable_db: &LockableDb,
) -> TantivyResult<tantivy::Index> {
    let mut schema_builder = Schema::builder();

    for field_name in &text_index.fields {
        schema_builder.add_text_field(field_name, TEXT);
    }

    // Add the _id field with STORED
    schema_builder.add_text_field(DEFAULT_ID_FIELD, STORED);

    let schema = schema_builder.build();
    let index_path = get_tantivy_index_path(&text_index.reference, lockable_db);
    let path = Path::new(&index_path);
    let index = tantivy::Index::create_in_dir(path, schema)?;

    Ok(index)
}
