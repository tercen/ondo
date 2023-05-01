// rocks_db_accessor/mod.rs

use super::LockableDb;
use rocksdb::{Options, DB};
use std::sync::{Arc, RwLock};
use tempfile::TempDir;

pub(crate) type DbArc = Arc<(RwLock<DB>, Option<TempDir>, String)>;

#[derive(Clone)]
pub(super) struct RocksDbAccessor {
    pub db_arc: DbArc,
    options: Options,
}

impl Default for RocksDbAccessor {
    fn default() -> Self {
        let db_path = std::env::var("ONDO_DB_PATH").unwrap_or("./db/ondo_rocksdb".to_owned());

        let mut options = Options::default();
        options.create_if_missing(true);

        Self::init(db_path, None, options)
    }
}

impl RocksDbAccessor {
    fn init(db_path: String, temp_dir: Option<TempDir>, options: Options) -> Self {
        let cf_names = DB::list_cf(&options, &db_path).unwrap_or(Vec::new());
        let raw_db = DB::open_cf(&options, &db_path, cf_names).unwrap();
        let db = Arc::new((RwLock::new(raw_db), temp_dir, db_path));

        RocksDbAccessor {
            db_arc: db,
            options,
        }
    }

    pub fn in_memory() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temporary directory");
        let db_path = temp_dir.path().to_string_lossy().into_owned();

        let mut options = Options::default();
        options.create_if_missing(true);

        Self::init(db_path, Some(temp_dir), options)
    }

    pub(crate) fn lockable_db(&self) -> LockableDb {
        LockableDb::new(self.db_arc.clone())
    }
}
