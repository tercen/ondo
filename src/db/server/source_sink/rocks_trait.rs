use crate::db::db_error::{DbError, DbResult};
use rocksdb::{Direction, IteratorMode, ReadOptions, DB};

type ResultBinaryPair = DbResult<(Vec<u8>, Vec<u8>)>;
type ResultBinaryPairIterator<'a> = DbResult<Box<dyn Iterator<Item = ResultBinaryPair> + 'a>>;

pub(super) trait RocksTrait<'a> {
    fn get_records_in_cf(&'a self, cf_name: &str) -> ResultBinaryPairIterator<'a>;
    fn get_records_in_cf_with_key_prefix_old(
        &'a self,
        value_cf_name: &str,
        key_prefix: Vec<u8>,
    ) -> DbResult<Box<dyn Iterator<Item = DbResult<(Vec<u8>, Vec<u8>)>> + 'a>>;
    fn get_records_in_cf_with_key_prefix(
        &'a self,
        value_cf_name: &str,
        key_prefix: Vec<u8>,
        start_key: Option<Vec<u8>>,
        page_size: Option<usize>,
    ) -> DbResult<Box<dyn Iterator<Item = DbResult<(Vec<u8>, Vec<u8>)>> + 'a>>;
    fn get_records_in_cf_with_key_range(
        &self,
        cf_name: &str,
        start_key: Vec<u8>,
        end_key: Vec<u8>,
    ) -> DbResult<Box<dyn Iterator<Item = DbResult<(Vec<u8>, Vec<u8>)>> + '_>>;
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
    fn get_records_in_cf_with_key_prefix_old(
        &'a self,
        value_cf_name: &str,
        key_prefix: Vec<u8>,
    ) -> DbResult<Box<dyn Iterator<Item = DbResult<(Vec<u8>, Vec<u8>)>> + 'a>> {
        self.get_records_in_cf_with_key_prefix(value_cf_name, key_prefix, None, None)
    }
    fn get_records_in_cf_with_key_prefix(
        &'a self,
        value_cf_name: &str,
        key_prefix: Vec<u8>,
        start_key: Option<Vec<u8>>,
        page_size: Option<usize>,
    ) -> DbResult<Box<dyn Iterator<Item = DbResult<(Vec<u8>, Vec<u8>)>> + 'a>> {
        let cf_handle = self.cf_handle(value_cf_name).ok_or(DbError::CfNotFound)?;
        let mut read_options = ReadOptions::default();
        read_options.set_prefix_same_as_start(true);
    
        let iterator_mode = if let Some(ref start_key) = start_key {
            IteratorMode::From(start_key, Direction::Forward)
        } else {
            IteratorMode::From(&key_prefix, Direction::Forward)
        };
    
        let raw_iterator = self.iterator_cf_opt(cf_handle, read_options, iterator_mode);
        let prefixed_iterator = raw_iterator.filter(move |result| {
            result
                .as_ref()
                .map(|(key, _)| key.starts_with(&key_prefix)) // Use the owned key_prefix
                .unwrap_or(false)
        });
    
        let iterator: Box<dyn Iterator<Item = DbResult<(Vec<u8>, Vec<u8>)>> + 'a> = if let Some(page_size) = page_size {
            Box::new(
                prefixed_iterator
                    .map(|result| {
                        result
                            .map(|(k, v)| (k.into_vec(), v.into_vec()))
                            .map_err(|err| DbError::RocksDbError(err))
                    })
                    .take(page_size + 1),
            )
        } else {
            Box::new(prefixed_iterator.map(|result| {
                result
                    .map(|(k, v)| (k.into_vec(), v.into_vec()))
                    .map_err(|err| DbError::RocksDbError(err))
            }))
        };
    
        Ok(iterator)
    }
    

    fn get_records_in_cf_with_key_range(
        &self,
        cf_name: &str,
        start_key: Vec<u8>,
        end_key: Vec<u8>,
    ) -> DbResult<Box<dyn Iterator<Item = DbResult<(Vec<u8>, Vec<u8>)>> + '_>> {
        let cf_handle = self.cf_handle(cf_name).ok_or(DbError::CfNotFound)?;
        let iter = self.iterator_cf(
            cf_handle,
            IteratorMode::From(&start_key, Direction::Forward),
        );
        let range_iterator = iter
            .take_while(move |res| match res {
                Ok((k, _)) => k.as_ref() <= end_key.as_slice(),
                Err(_) => true,
            })
            .map(|result| {
                result
                    .map_err(|e| DbError::Other(e.to_string()))
                    .map(|(k, v)| (k.to_vec(), v.to_vec()))
            });

        Ok(Box::new(range_iterator))
    }
}
