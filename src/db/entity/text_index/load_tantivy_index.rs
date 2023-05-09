use super::get_tantivy_index_path::get_tantivy_index_path;
use super::TextIndex;
use crate::db::server::lockable_db::transaction_maker::TransactionMaker;
use std::path::Path;
use tantivy::Result as TantivyResult;

pub(super) fn load_tantivy_index(text_index: &TextIndex, lockable_db: &TransactionMaker) -> TantivyResult<tantivy::Index> {
    let index_path = get_tantivy_index_path(&text_index.reference, lockable_db);
    let path = Path::new(&index_path);
    tantivy::Index::open_in_dir(&path)
}
