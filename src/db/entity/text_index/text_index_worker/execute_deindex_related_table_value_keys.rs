// text_index_worker/execute_deindex_related_table_value_keys.rs
use super::TextIndexWorker;
use crate::db::entity::table_value::get_key_from_table_value;
use crate::db::reference::table_reference::TableReferenceTrait;
use crate::db::server::lockable_db::LOCKABLE_DB;

impl TextIndexWorker {
    pub(crate) fn execute_deindex_related_table_value_keys(&self) -> Result<(), String> {
        let table_reference = &self.text_index.reference.table_reference;
        let db_read_lock = LOCKABLE_DB.read();

        let all_values_iterator = table_reference
            .all_values(&db_read_lock)
            .map_err(|e| e.to_string())?;

        let mut writer = self
            .tantivy_index
            .writer(50_000_000)
            .map_err(|e| e.to_string())?;

        for table_value_result in all_values_iterator {
            match table_value_result {
                Ok(table_value) => {
                    let ondo_key = get_key_from_table_value(&table_value);
                    self.deindex_table_value_key_with_writer(&ondo_key, &mut writer)?;
                }
                Err(e) => {
                    return Err(e.to_string());
                }
            }
        }

        writer.commit().map_err(|e| e.to_string())?;
        Ok(())
    }
}
