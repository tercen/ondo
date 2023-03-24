use crate::db::db_error::{DbError, DbResult};
use rocksdb::{IteratorMode, DB};

type ResultBinaryPair = DbResult<(Vec<u8>, Vec<u8>)>;
type ResultBinaryPairIterator<'a> = DbResult<Box<dyn Iterator<Item = ResultBinaryPair> + 'a>>;

pub(super) trait RocksTrait<'a> {
    fn get_records_in_cf(&'a self, cf_name: &str) -> ResultBinaryPairIterator<'a>;
}

impl<'a> RocksTrait<'a> for DB {
    fn get_records_in_cf(&'a self, cf_name: &str) -> ResultBinaryPairIterator<'a> {
        let cf_handle = self.cf_handle(cf_name).ok_or(DbError::CfNotFound)?;

        let iter = self.iterator_cf(cf_handle, IteratorMode::Start);

        let new_iter = iter.map(|kv_result| {
            kv_result
                .map(|(k, v)| (k.to_vec(), v.to_vec()))
                .map_err(|err| DbError::RocksDbError(err))
        });

        let boxed_new_iter = Box::new(new_iter);
        Ok(boxed_new_iter)
    }
}
