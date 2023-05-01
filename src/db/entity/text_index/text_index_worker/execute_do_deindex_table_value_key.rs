// text_index_worker/execute_do_deindex_table_value_key.rs
use super::TextIndexWorker;
use crate::db::entity::index::DEFAULT_ID_FIELD;
use crate::db::entity::ondo_key::OndoKey;
use tantivy::schema::Term;
use tantivy::IndexWriter;

impl TextIndexWorker {
    pub(crate) fn execute_do_deindex_table_value_key(&self, key: &OndoKey) -> Result<(), String> {
        let mut writer = self
            .tantivy_index
            .writer(50_000_000)
            .map_err(|e| e.to_string())?;
        self.deindex_table_value_key_with_writer(key, &mut writer)?;
        writer.commit().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub(super) fn deindex_table_value_key_with_writer(
        &self,
        key: &OndoKey,
        writer: &mut IndexWriter,
    ) -> Result<(), String> {
        let schema = self.tantivy_index.schema();
        let id_field = schema
            .get_field(DEFAULT_ID_FIELD)
            .ok_or_else(|| format!("Field not found in schema: {}", DEFAULT_ID_FIELD))?;
        let id_json_string = serde_json::to_string(key).map_err(|e| e.to_string())?;
        let term = Term::from_field_text(id_field, &id_json_string);

        writer.delete_term(term);
        Ok(())
    }
}
