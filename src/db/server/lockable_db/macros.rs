#[macro_export]
macro_rules! lockable_db_read {
    ($lockable_db:expr, $block:block) => {{
        let db_guard = lockable_db.read().await;
        let db = &*db_guard;
        let transaction_or_db =
            TransactionOrDb::Db(db);
        $block
    }};
}

#[macro_export]
macro_rules! lockable_db_write {
    ($lockable_db:expr, $block:block) => {{
        let db_guard = lockable_db.write().await;
        let mut db = &*db_guard;
        let transaction_or_db =
            TransactionOrDb::Db(db);
        $block
    }};
}

#[macro_export]
macro_rules! lockable_db_transaction {
    ($lockable_db:expr, $block:block) => {{
        let db_guard = lockable_db.write().await;
        let db = &*db_guard;
        let transaction = db.transaction();
        let transaction_or_db =
        TransactionOrDb::Transaction(
                &transaction,
                db,
            );
        $block
    }};
}
