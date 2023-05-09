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

//FIXME: Drop for DbArc should be Drop for DbData
impl<'a> Drop for DbArc {
    fn drop(&mut self) {
        // Dropping `temp_dir` before `transaction_or_db`
        if let Some(temp_dir) = self.db_lock.temp_dir.take() {
            // Dropping `temp_dir`
            drop(temp_dir);
        }
        // Dropping `transaction_or_db` implicitly
    }
}
