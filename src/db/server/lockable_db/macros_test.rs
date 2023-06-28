#[cfg(test)]
mod test {
    use crate::db::server::lockable_db::{LockableDb, LOCKABLE_DB};
    use crate::*;

    async fn example_db_access() {
        let lockable_db = LockableDb::in_memory();
        let db_tuple = lockable_db_read!(lockable_db, {});
    }

    async fn example_transaction_access() {
        let lockable_db = LockableDb::in_memory();
        let db_tuple = lockable_db_transaction!(lockable_db, {transaction.commit();});
    }

    async fn use_case_transaction() {
        let lockable_db = LOCKABLE_DB.clone();
        lockable_db_transaction!(lockable_db, {});
    }
    async fn use_case_db() {
        let lockable_db = LOCKABLE_DB.clone();
        lockable_db_read!(lockable_db, {});
    }
    async fn use_case_mut_db() {
        let lockable_db = LOCKABLE_DB.clone();
        lockable_db_write!(lockable_db, {});
    }
}
