use super::db_error_to_status::DbErrorOptionToStatus;
use super::db_error_to_status::DbErrorToStatus;
use super::index_server_trait::IndexServerTrait;
use super::rocks_db_accessor::DbReadLockGuardWrapper;
use super::rocks_db_accessor::RocksDbAccessor;
use super::source_sink::effects_sink::EffectsSink;
use crate::db::{
    entity::{index::Index, OndoKey, TableValue},
    reference::index_reference::{IndexReference, IndexReferenceTrait},
    DbError,
};
use crate::ondo_remote;
use ondo_remote::*;
use tonic::{Request, Response, Status};

impl<'a> Into<IndexReference> for &'a IndexReferenceMessage {
    fn into(self) -> IndexReference {
        IndexReference {
            table_reference: self.table_reference.as_ref().unwrap().into(),
            index_name: self.index_name.clone(),
        }
    }
}
impl Into<IndexReferenceMessage> for IndexReference {
    fn into(self) -> IndexReferenceMessage {
        IndexReferenceMessage {
            table_reference: Some(self.table_reference.into()),
            index_name: self.index_name,
        }
    }
}
impl<'a> Into<Index> for &'a IndexMessage {
    fn into(self) -> Index {
        let reference: IndexReference = self.index_reference.as_ref().unwrap().into();
        let fields: Vec<String> = self.fields.clone();
        Index {
            fields: fields,
            reference,
        }
    }
}
impl Into<IndexMessage> for Index {
    fn into(self) -> IndexMessage {
        let reference: IndexReferenceMessage = self.reference.into();
        let fields: Vec<String> = self.fields.clone();
        IndexMessage {
            fields: fields,
            index_reference: Some(reference),
        }
    }
}

struct IndexedValueReference {
    index_reference: IndexReference,
    key: OndoKey,
}
impl<'a> Into<IndexedValueReference> for &'a IndexedValueReferenceMessage {
    fn into(self) -> IndexedValueReference {
        IndexedValueReference {
            index_reference: self.index_reference.as_ref().unwrap().into(),
            key: self.key.as_ref().unwrap().into(),
        }
    }
}

struct IndexedValueRangeReference {
    index_reference: IndexReference,
    start_key: OndoKey,
    end_key: OndoKey,
}
impl<'a> Into<IndexedValueRangeReference> for &'a IndexedValueRangeReferenceMessage {
    fn into(self) -> IndexedValueRangeReference {
        IndexedValueRangeReference {
            index_reference: self.index_reference.as_ref().unwrap().into(),
            start_key: self.start_key.as_ref().unwrap().into(),
            end_key: self.end_key.as_ref().unwrap().into(),
        }
    }
}

impl IndexServerTrait for RocksDbAccessor {
    fn create_index(&self, r: Request<IndexMessage>) -> Result<Response<EmptyMessage>, Status> {
        let entity: Index = r.get_ref().into();
        entity
            .reference
            .post_index(&entity, self)
            .map_db_err_to_status()?
            .apply_effects(self)
    }

    fn delete_index(
        &self,
        r: Request<IndexReferenceMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        let reference: IndexReference = r.get_ref().into();
        reference
            .delete_index(self)
            .map_db_err_to_status()?
            .apply_effects(self)
    }

    fn get_index(
        &self,
        r: Request<IndexReferenceMessage>,
    ) -> Result<Response<IndexMessage>, Status> {
        let reference: IndexReference = r.get_ref().into();
        reference
            .get_index(self)
            .map_db_err_option_to_status()
            .map(|entity| Response::new(entity.into()))
    }

    fn update_index(&self, r: Request<IndexMessage>) -> Result<Response<EmptyMessage>, Status> {
        let entity: Index = r.get_ref().into();
        entity
            .reference
            .put_index(&entity, self)
            .map_db_err_to_status()?
            .apply_effects(self)
    }

    fn find_values(
        &self,
        r: Request<IndexedValueReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status> {
        let guarded_db = self.guarded_db();
        let db_wrapper = DbReadLockGuardWrapper::new(&guarded_db).map_db_err_to_status()?;
        let indexed_value_reference: IndexedValueReference = r.get_ref().into();
        let reference = indexed_value_reference.index_reference;
        let key_prefix = indexed_value_reference.key;
        let iterator = reference
            .all_values_with_key_prefix(key_prefix, self, &db_wrapper)
            .map_db_err_to_status()?;
        let values_result: Result<Vec<TableValue>, DbError> = iterator.collect();
        let values = values_result.map_db_err_to_status()?;
        let json = serde_json::to_string(&values).map_err(|e| Status::internal(e.to_string()))?;
        let response = Response::new(JsonMessage { json });
        Ok(response)
    }

    fn find_values_by_range(
        &self,
        r: Request<IndexedValueRangeReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status> {
        let guarded_db = self.guarded_db();
        let db_wrapper = DbReadLockGuardWrapper::new(&guarded_db).map_db_err_to_status()?;
        let indexed_value_range_reference: IndexedValueRangeReference = r.get_ref().into();
        let reference = indexed_value_range_reference.index_reference;
        let start_key_prefix = indexed_value_range_reference.start_key;
        let end_key_prefix = indexed_value_range_reference.end_key;
        let iterator = reference
            .all_values_with_key_range(start_key_prefix, end_key_prefix, self, &db_wrapper)
            .map_db_err_to_status()?;
        let values_result: Result<Vec<TableValue>, DbError> = iterator.collect();
        let values = values_result.map_db_err_to_status()?;
        let json = serde_json::to_string(&values).map_err(|e| Status::internal(e.to_string()))?;
        let response = Response::new(JsonMessage { json });
        Ok(response)
    }
}
