// db_arc.rs
use rocksdb::TransactionDB;
use std::sync::{Arc, RwLock};
use tempfile::TempDir;
#[derive(Clone)]
pub(crate) struct DbArc {
    pub db_lock: Arc<DbData>,
}

impl DbArc {
    pub fn new(db: TransactionDB, temp_dir: Option<TempDir>, db_path: String) -> Self {
        // let transaction_or_db = TransactionOrDb::new(db);
        let db_data = DbData {
            db: RwLock::new(db),
            temp_dir,
            db_path,
        };
        DbArc {
            db_lock: Arc::new(db_data),
        }
    }
}

pub(crate) struct DbData {
    pub db: RwLock<TransactionDB>,
    pub temp_dir: Option<TempDir>,
    pub db_path: String,
}

// impl<'a> Drop for DbData {
//     fn drop(&mut self) {
//         // Dropping `db` before `temp_dir`
//         // if let Some(db) = self.db.take() {
//             // Dropping `db`
//             drop(self.db);
//         // }
//         // Dropping `temp_dir` implicitly
//     }
// }
