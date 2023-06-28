use rocksdb::{Transaction, TransactionDB};

#[derive(Clone)]
pub enum TransactionOrDb<'a> {
    Transaction(&'a Transaction<'a, TransactionDB>, &'a TransactionDB),
    Db(&'a TransactionDB),
}

pub(crate) type TransactionDBIterator<'a> = rocksdb::DBIteratorWithThreadMode<'a, TransactionDB>;

/*
  FIXME:
  This idea is technically wrong. There should be 4 traits: BaseOndoDb, OndoDb, OndoTransaction, and MetaOndoDb.
  And those traits should be used for static dispatch. However, due to time constraints, we will use the enum.
  However, enum is leading to more and more lifecycle issues. Going on brute force.<
  Those lifecycle issues would not happen on static dispatch.
  Umur
*/
impl<'a> TransactionOrDb<'a> {
    //FIXME: transaction needs cf_handle
    pub(crate) fn cf_handle(&self, cf_name: &str) -> Option<&rocksdb::ColumnFamily> {
        match self {
            TransactionOrDb::Transaction(_transaction, db) => db.cf_handle(cf_name),
            TransactionOrDb::Db(db) => db.cf_handle(cf_name),
        }
    }

    // FIXME: Add get for update
    pub(crate) fn get_cf(
        &self,
        cf: &rocksdb::ColumnFamily,
        key: &Vec<u8>,
    ) -> Result<Option<Vec<u8>>, rocksdb::Error> {
        match self {
            TransactionOrDb::Transaction(transaction, _) => transaction.get_cf(cf, key),
            TransactionOrDb::Db(db) => db.get_cf(cf, key),
        }
    }
    pub(crate) fn get_for_update_cf(
        &self,
        cf: &rocksdb::ColumnFamily,
        key: &Vec<u8>,
    ) -> Result<Option<Vec<u8>>, rocksdb::Error> {
        match self {
            TransactionOrDb::Transaction(transaction, _) => {
                transaction.get_for_update_cf(cf, key, true)
            }
            TransactionOrDb::Db(db) => db.get_cf(cf, key),
        }
    }
    pub(crate) fn put_cf(
        &self,
        cf: &rocksdb::ColumnFamily,
        key: Vec<u8>,
        value: Vec<u8>,
    ) -> Result<(), rocksdb::Error> {
        match self {
            TransactionOrDb::Transaction(transaction, _) => transaction.put_cf(cf, key, value),
            TransactionOrDb::Db(db) => db.put_cf(cf, key, value),
        }
    }

    pub(crate) fn delete_cf(
        &self,
        cf: &rocksdb::ColumnFamily,
        key: Vec<u8>,
    ) -> Result<(), rocksdb::Error> {
        match self {
            TransactionOrDb::Transaction(transaction, _) => transaction.delete_cf(cf, key),
            TransactionOrDb::Db(db) => db.delete_cf(cf, key),
        }
    }

    pub(crate) fn iterator_cf(
        &self,
        cf: &rocksdb::ColumnFamily,
        iterator_mode: rocksdb::IteratorMode,
    ) -> TransactionDBIterator {
        let db = match self {
            TransactionOrDb::Transaction(_transaction, db) => db,
            TransactionOrDb::Db(db) => db,
        };
        db.iterator_cf(cf, iterator_mode)
    }

    pub(crate) fn iterator_cf_opt(
        &self,
        cf: &rocksdb::ColumnFamily,
        read_options: rocksdb::ReadOptions,
        iterator_mode: rocksdb::IteratorMode,
    ) -> TransactionDBIterator {
        let db = match self {
            TransactionOrDb::Transaction(_transaction, db) => db,
            TransactionOrDb::Db(db) => db,
        };
        db.iterator_cf_opt(cf, read_options, iterator_mode)
    }

}

unsafe impl<'a> Send for TransactionOrDb<'a> where &'a TransactionDB: Send {}
//FIXME To remove unsafe Send we need to put transaction under Arc after creating it.
