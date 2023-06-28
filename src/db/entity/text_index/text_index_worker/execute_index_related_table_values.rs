// text_index_worker/execute_index_related_table_values.rs
use super::TextIndexWorker;
use crate::db::reference::table_reference::TableReferenceTrait;
use crate::db::server::lockable_db::LOCKABLE_DB;
use crate::db::server::lockable_db::transaction_or_db::TransactionOrDb;

impl TextIndexWorker {
    pub(crate) async fn execute_index_related_table_values(&self) -> Result<(), String> {
        let table_reference = &self.text_index.reference.table_reference;

        let lockable_db = LOCKABLE_DB.clone();

        let db_guard = lockable_db.read().await;
        let db = &*db_guard;
        let transaction_or_db =
            TransactionOrDb::Db(db);

        let all_values_iterator = table_reference
            .all_values(&transaction_or_db)
            .map_err(|e| e.to_string())?;

        let mut writer = self
            .tantivy_index
            .writer(50_000_000)
            .map_err(|e| e.to_string())?;

        for table_value_result in all_values_iterator {
            match table_value_result {
                Ok(table_value) => {
                    self.index_table_value_with_writer(&table_value, &mut writer)?;
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
