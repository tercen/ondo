// text_index/mod.rs
mod create_tantivy_index;
mod get_tantivy_index_path;
mod load_or_create_tantivy_index;
mod load_tantivy_index;

pub(crate) mod search_table_values;
pub(crate) mod text_index_task;
pub(crate) mod text_index_worker;

use crate::db::tasks::task::Task;
use crate::db::{
    entity::ondo_key::OndoKey, entity::table_value::TableValue,
    reference::text_index_reference::TextIndexReference,
};
use serde::{Deserialize, Serialize};
use text_index_task::TextIndexTask;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub(crate) struct TextIndex {
    pub reference: TextIndexReference,
    pub fields: Vec<String>,
}

impl TextIndex {
    pub(crate) fn index_related_table_values(&self) -> Task {
        Task::from_text_index_task(TextIndexTask::IndexRelatedTableValues(self.clone()))
    }

    pub(crate) fn deindex_related_table_value_keys(&self) -> Task {
        Task::from_text_index_task(TextIndexTask::DeindexRelatedTableValueKeys(self.clone()))
    }

    pub(crate) fn do_index_table_value(&self, value: &TableValue) -> Task {
        Task::from_text_index_task(TextIndexTask::DoIndexTableValue(
            self.clone(),
            value.clone(),
        ))
    }

    pub(crate) fn do_deindex_table_value_key(&self, key: &OndoKey) -> Task {
        Task::from_text_index_task(TextIndexTask::DoDeindexTableValueKey(
            self.clone(),
            key.clone(),
        ))
    }
}
