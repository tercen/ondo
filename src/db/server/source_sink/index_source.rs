//index_source.rs
use super::rocks_trait::RocksTrait;
use crate::db::db_error::DbResult;
use crate::db::entity::index::IndexValue;
use crate::db::entity::ondo_key::OndoKey;
use crate::db::entity::reference::requests::index_requests::IndexIteratorRequests;
use crate::db::server::rocks_db_accessor::DbReadLockGuardWrapper;
use crate::db::server::source_sink::ondo_serializer::OndoSerializer;
use serde_json::Value;

// Implement IndexIteratorRequests for DbReadLockGuardWrapper
impl<'a> IndexIteratorRequests<'a> for DbReadLockGuardWrapper<'a> {
    fn all_values_with_key_prefix(
        &'a self,
        value_cf_name: &str,
        key_prefix: OndoKey,
    ) -> DbResult<Box<dyn Iterator<Item = DbResult<IndexValue>> + 'a>> {
        let serialized_key_prefix = key_prefix.ondo_serialize()?;
        let raw_iterator = self
            .guard
            .get_records_in_cf_with_key_prefix_old(value_cf_name, serialized_key_prefix)?;

        let all_iterator = raw_iterator.map(|result| {
            result.and_then(|(_, v)| Value::ondo_deserialize(&v)) // Flatten the nested Result
        });

        let ok_iterator = Box::new(all_iterator);
        Ok(ok_iterator)
    }

    fn all_values_with_key_range(
        &'a self,
        value_cf_name: &str,
        start_key_prefix: OndoKey,
        end_key_prefix: OndoKey,
    ) -> DbResult<Box<dyn Iterator<Item = DbResult<IndexValue>> + 'a>> {
        let serialized_start_key_prefix = start_key_prefix.ondo_serialize()?;
        let serialized_end_key_prefix = end_key_prefix.ondo_serialize()?;
        let raw_iterator = self.guard.get_records_in_cf_with_key_range(
            value_cf_name,
            serialized_start_key_prefix,
            serialized_end_key_prefix,
        )?;

        let all_iterator = raw_iterator.map(|result| {
            result.and_then(|(_, v)| Value::ondo_deserialize(&v)) // Flatten the nested Result
        });

        let ok_iterator = Box::new(all_iterator);
        Ok(ok_iterator)
    }
}
