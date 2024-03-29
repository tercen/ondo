// text_index/text_index_task.rs
use super::text_index_worker::TextIndexWorker;
use super::TextIndex;
use crate::db::entity::table_value::TableValue;
use crate::db::server::lockable_db::LOCKABLE_DB;

use crate::db::entity::ondo_key::OndoKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub(crate) enum TextIndexTask {
    IndexRelatedTableValues(TextIndex),
    DeindexRelatedTableValueKeys(TextIndex),
    DoIndexTableValue(TextIndex, TableValue),
    DoDeindexTableValueKey(TextIndex, OndoKey),
}

impl TextIndexTask {
    pub async fn execute(&self) -> Result<(), String> {
        let lockable_db = LOCKABLE_DB.clone();

        match self {
            TextIndexTask::IndexRelatedTableValues(text_index) => {
                let worker = TextIndexWorker::from_text_index(text_index.clone(), lockable_db)?;
                worker.execute_index_related_table_values().await
            }
            TextIndexTask::DeindexRelatedTableValueKeys(text_index) => {
                let worker = TextIndexWorker::from_text_index(text_index.clone(), lockable_db)?;
                worker.execute_deindex_related_table_value_keys().await
            }
            TextIndexTask::DoIndexTableValue(text_index, value) => {
                let worker = TextIndexWorker::from_text_index(text_index.clone(), lockable_db)?;
                worker.execute_do_index_table_value(value)
            }
            TextIndexTask::DoDeindexTableValueKey(text_index, key) => {
                let worker = TextIndexWorker::from_text_index(text_index.clone(), lockable_db)?;
                worker.execute_do_deindex_table_value_key(key)
            }
        }
    }
}
