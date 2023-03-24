use super::db_error_to_status::DbErrorOptionToStatus;
use super::db_error_to_status::DbErrorToStatus;
use super::index_server_trait::IndexServerTrait;
use super::rocks_db_accessor::RocksDbAccessor;
use super::source_sink::effects_sink::EffectsSink;
use crate::db::entity::index::Index;
use crate::db::entity::reference::index_reference::IndexReference;
use crate::db::entity::reference::index_reference::IndexReferenceTrait;
use crate::ondo_remote::*;
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
}
