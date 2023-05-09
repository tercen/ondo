// text_index_server_trait_impl.rs
use super::{
    db_error_to_status::{DbErrorOptionToStatus, DbErrorToStatus},
    lockable_db::transaction_maker::TransactionMaker,
    source_sink::effects_sink::EffectsTasksSink,
};
use crate::db::{entity::TableValue, server::text_index_server_trait::TextIndexServerTrait};
use crate::ondo_remote::*;
use tonic::{Request, Response, Status};

use crate::db::entity::text_index::TextIndex;
use crate::db::reference::text_index_reference::TextIndexReference;

impl<'a> Into<TextIndexReference> for &'a TextIndexReferenceMessage {
    fn into(self) -> TextIndexReference {
        TextIndexReference {
            table_reference: self.table_reference.as_ref().unwrap().into(),
            index_name: self.index_name.clone(),
        }
    }
}
impl Into<TextIndexReferenceMessage> for TextIndexReference {
    fn into(self) -> TextIndexReferenceMessage {
        TextIndexReferenceMessage {
            table_reference: Some(self.table_reference.into()),
            index_name: self.index_name,
        }
    }
}
impl<'a> Into<TextIndex> for &'a TextIndexMessage {
    fn into(self) -> TextIndex {
        let reference: TextIndexReference = self.text_index_reference.as_ref().unwrap().into();
        let fields: Vec<String> = self.fields.clone();
        TextIndex {
            fields: fields,
            reference,
        }
    }
}
impl Into<TextIndexMessage> for TextIndex {
    fn into(self) -> TextIndexMessage {
        let reference: TextIndexReferenceMessage = self.reference.into();
        let fields: Vec<String> = self.fields.clone();
        TextIndexMessage {
            fields: fields,
            text_index_reference: Some(reference),
        }
    }
}

struct TantivyQuery {
    reference: TextIndexReference,
    query: String,
    page_size: Option<usize>,
    page_number: Option<usize>,
}
impl<'a> Into<TantivyQuery> for &'a TantivyQueryMessage {
    fn into(self) -> TantivyQuery {
        TantivyQuery {
            reference: self.text_index_reference.as_ref().unwrap().into(),
            query: self.query.clone(),
            page_number: self.optional_page_number.map(|n| n.try_into().unwrap()),
            page_size: self.optional_page_size.map(|n| n.try_into().unwrap()),
        }
    }
}

impl<'a> TextIndexServerTrait for TransactionMaker<'a> {
    fn create_text_index(
        &self,
        r: Request<TextIndexMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        let entity: TextIndex = r.get_ref().into();
        entity
            .reference
            .post_text_index(&entity, self)
            .map_db_err_to_status()?
            .apply_effects_queue_tasks(self)
    }

    fn delete_text_index(
        &self,
        r: Request<TextIndexReferenceMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        let reference: TextIndexReference = r.get_ref().into();
        reference
            .delete_text_index(self)
            .map_db_err_to_status()?
            .apply_effects_queue_tasks(self)
    }

    fn get_text_index(
        &self,
        r: Request<TextIndexReferenceMessage>,
    ) -> Result<Response<TextIndexMessage>, Status> {
        let reference: TextIndexReference = r.get_ref().into();
        reference
            .get_text_index(self)
            .map_db_err_option_to_status()
            .map(|entity| Response::new(entity.into()))
    }

    fn update_text_index(
        &self,
        r: Request<TextIndexMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        let entity: TextIndex = r.get_ref().into();
        entity
            .reference
            .put_text_index(&entity, self)
            .map_db_err_to_status()?
            .apply_effects_queue_tasks(self)
    }

    fn search_text_index(
        &self,
        r: Request<TantivyQueryMessage>,
    ) -> Result<Response<JsonMessage>, Status> {
        let tantivy_query: TantivyQuery = r.get_ref().into();
        let reference = tantivy_query.reference;
        let text_index = reference
            .get_text_index(self)
            .map_db_err_option_to_status()?;
        let iterator = text_index
            .search_iterator(
                &tantivy_query.query,
                tantivy_query.page_size,
                tantivy_query.page_number,
                self,
            )
            .map_db_err_to_status()?;

        let values: Vec<TableValue> = iterator.collect();
        let json = serde_json::to_string(&values).map_err(|e| Status::internal(e.to_string()))?;
        let response = Response::new(JsonMessage { json });
        Ok(response)
    }
}
