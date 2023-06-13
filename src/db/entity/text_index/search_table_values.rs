use super::load_tantivy_index::load_tantivy_index;

use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;

use super::TextIndex;
use crate::db::db_error::DbError;
use crate::db::entity::index::DEFAULT_ID_FIELD;
use crate::db::entity::ondo_key::OndoKey;
use crate::db::entity::table_value::TableValue;
use crate::db::reference::table_value_reference::{TableValueReference, TableValueReferenceTrait};
use crate::db::server::lockable_db::transaction_maker::TransactionMaker;

impl TextIndex {
    pub(crate) fn search_vec<'a>(
        &'a self,
        query_string: &'a str,
        page_size: Option<usize>,
        page_number: Option<usize>,
        lockable_db: &'a TransactionMaker<'a>,
    ) -> Result<Vec<TableValue>, DbError> {
        self.search_iterator(query_string, page_size, page_number, lockable_db)
            .map(|results| results.collect())
    }

    pub(crate) fn search_iterator<'a>(
        &'a self,
        query_string: &'a str,
        page_size: Option<usize>,
        page_number: Option<usize>,
        lockable_db: &'a TransactionMaker<'a>,
    ) -> Result<impl Iterator<Item = TableValue> + 'a, DbError> {
        let index = load_tantivy_index(&self, lockable_db)
            .map_err(|e| DbError::TantivyError(e.to_string()))?;
        let schema = index.schema();
        let fields: Vec<_> = schema.fields().map(|(field, _)| field).collect();

        let query_parser = QueryParser::for_index(&index, fields);
        let query = query_parser
            .parse_query(query_string)
            .map_err(|e| DbError::TantivyError(e.to_string()))?;

        let searcher = index
            .reader()
            .map_err(|e| DbError::TantivyError(e.to_string()))?
            .searcher();
        let top_docs = searcher
            .search(&query, &TopDocs::with_limit(10000))
            .map_err(|e| DbError::TantivyError(e.to_string()))?;

        let start = page_size
            .map(|page_size| page_size * page_number.unwrap_or(0))
            .unwrap_or(0);
        let end = page_size
            .map(|page_size| start + page_size)
            .unwrap_or(top_docs.len());

        let iter = (start..end).filter_map(move |i| {
            let (_, doc_address) = top_docs[i];
            let stored_doc = searcher.doc(doc_address).ok()?;
            let id_field = schema.get_field(DEFAULT_ID_FIELD)?;
            let id_opt = stored_doc.get_first(id_field)?.as_text().to_owned();
            let id = id_opt?;
            let ondo_key: OndoKey = serde_json::from_str(id).ok()?;
            let table_value_ref =
                TableValueReference::new(self.reference.table_reference.clone(), ondo_key.clone());
            table_value_ref.get_table_value(lockable_db).ok().flatten()
        });

        Ok(iter)
    }
}
