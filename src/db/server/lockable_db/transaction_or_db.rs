use rocksdb::{Transaction, TransactionDB};

pub(crate) enum TransactionOrDb<'a> {
    Transaction(&'a Transaction<'a, TransactionDB>, &'a TransactionDB),
    Db(&'a TransactionDB),
}

pub(crate) enum MutTransactionOrDb<'a> {
    Transaction,
    Db(&'a mut TransactionDB),
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
            TransactionOrDb::Transaction(transaction, db) => db.cf_handle(cf_name),
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

    // FIXME: We wont support transaction iterators
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

    // FIXME: We wont support transaction iterators
    pub(crate) fn iterator_cf(
        &self,
        cf: &rocksdb::ColumnFamily,
        iterator_mode: rocksdb::IteratorMode,
    ) -> TransactionDBIterator {
        let db = match self {
            TransactionOrDb::Transaction(transaction, db) => db,
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
            TransactionOrDb::Transaction(transaction, db) => db,
            TransactionOrDb::Db(db) => db,
        };
        db.iterator_cf_opt(cf, read_options, iterator_mode)
    }
}

/*
Not supported by transactions
*/

impl<'a> MutTransactionOrDb<'a> {
    pub(crate) fn create_cf(
        &mut self,
        cf_name: &String,
        cf_opts: &rocksdb::Options,
    ) -> Result<(), rocksdb::Error> {
        match self {
            MutTransactionOrDb::Transaction => unimplemented!(),
            MutTransactionOrDb::Db(db) => db.create_cf(cf_name, cf_opts),
        }
    }

    pub(crate) fn drop_cf(&mut self, cf_name: &String) -> Result<(), rocksdb::Error> {
        match self {
            MutTransactionOrDb::Transaction => unimplemented!(),
            MutTransactionOrDb::Db(db) => db.drop_cf(cf_name),
        }
    }
}
