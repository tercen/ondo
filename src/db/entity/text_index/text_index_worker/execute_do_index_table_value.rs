// text_index_worker/execute_do_index_table_value.rs
use super::TextIndexWorker;
use crate::db::entity::index::{get_nested_property, DEFAULT_ID_FIELD};
use crate::db::entity::table_value::TableValue;
use tantivy::{Document, IndexWriter};

impl<'a> TextIndexWorker<'a> {
    pub(crate) fn execute_do_index_table_value(&self, value: &TableValue) -> Result<(), String> {
        let mut writer = self
            .tantivy_index
            .writer(50_000_000)
            .map_err(|e| e.to_string())?;
        self.index_table_value_with_writer(value, &mut writer)?;
        writer.commit().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub(super) fn index_table_value_with_writer(
        &self,
        value: &TableValue,
        writer: &mut IndexWriter,
    ) -> Result<(), String> {
        let schema = self.tantivy_index.schema();
        let mut doc = Document::default();

        for field_name in &self.text_index.fields {
            if let Some(field) = schema.get_field(field_name) {
                let value_from_table = get_nested_property(value, field_name);
                doc.add_text(field, &value_from_table.to_string());
            } else {
                return Err(format!("Field not found in schema: {}", field_name));
            }
        }

        if let Some(id_field) = schema.get_field(DEFAULT_ID_FIELD) {
            let id_value = value[DEFAULT_ID_FIELD].clone();
            let id_json_string = id_value.to_string();
            doc.add_text(id_field, &id_json_string);
        } else {
            return Err(format!("Field not found in schema: {}", DEFAULT_ID_FIELD));
        }

        writer.add_document(doc).map_err(|e| e.to_string())?;
        Ok(())
    }
}
