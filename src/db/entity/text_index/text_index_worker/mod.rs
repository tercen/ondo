mod execute_deindex_related_table_value_keys;
mod execute_do_deindex_table_value_key;
mod execute_do_index_table_value;
mod execute_index_related_table_values;

use crate::db::server::lockable_db::LockableDb;

use super::{load_or_create_tantivy_index::load_or_create_tantivy_index, TextIndex};

use std::sync::Arc;

pub(crate) struct TextIndexWorker {
    text_index: TextIndex,
    tantivy_index: Arc<tantivy::Index>,
    lockable_db: LockableDb,
}

impl TextIndexWorker {
    pub fn from_text_index(text_index: TextIndex, lockable_db: LockableDb) -> Result<Self, String> {
        let tantivy_index: tantivy::Index =
            load_or_create_tantivy_index(&text_index, &lockable_db).map_err(|e| e.to_string())?;
        Ok(TextIndexWorker {
            text_index: text_index.clone(),
            tantivy_index: Arc::new(tantivy_index),
            lockable_db,
        })
    }
}
