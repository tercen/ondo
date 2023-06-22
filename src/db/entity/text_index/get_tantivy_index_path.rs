// text_index/get_tantivy_index_path.rs
use crate::db::reference::text_index_reference::TextIndexReference;
use crate::db::server::lockable_db::transaction_maker::LockableTransactionOrDb;
use std::path::PathBuf;

pub(crate) fn get_tantivy_index_path(
    text_index_reference: &TextIndexReference,
    lockable_db: &LockableTransactionOrDb,
) -> PathBuf {
    let domain_name = &text_index_reference
        .table_reference
        .domain_reference
        .domain_name;
    let table_name = &text_index_reference.table_reference.table_name;
    let index_name = &text_index_reference.index_name;

    let db_path = lockable_db.db_path();

    let mut path = PathBuf::from(db_path);
    path.push(domain_name);
    path.push(table_name);
    path.push(index_name);

    path
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::server::lockable_db::{ LockableDb};

    #[test]
    fn test_get_tantivy_index_path() {
        let text_index_reference =
            TextIndexReference::build("test_domain", "test_table", "test_index");

        let lockable_db = LockableTransactionOrDb::with_db(LockableDb::in_memory());

        let db_path = lockable_db.db_path();
        let mut expected_path = PathBuf::from(db_path);
        expected_path.push("test_domain");
        expected_path.push("test_table");
        expected_path.push("test_index");

        let path = get_tantivy_index_path(&text_index_reference, &lockable_db);

        assert_eq!(path, expected_path);
    }
}
