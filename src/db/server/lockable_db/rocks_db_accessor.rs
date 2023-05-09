// rocks_db_accessor.rs
use super::db_arc::DbArc;
use rocksdb::{Options, TransactionDB, TransactionDBOptions};
use tempfile::TempDir;

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
        let txn_db_options = TransactionDBOptions::default();
        let db = TransactionDB::open(&options, &txn_db_options, &db_path).unwrap();
        let db_arc: DbArc = DbArc::new(db, temp_dir, db_path);

        RocksDbAccessor { db_arc, options }
    }

    pub fn in_memory() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temporary directory");
        let db_path = temp_dir.path().to_string_lossy().into_owned();

        let mut options = Options::default();
        options.create_if_missing(true);

        Self::init(db_path, Some(temp_dir), options)
    }
}
