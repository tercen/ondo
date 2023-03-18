use super::db_error_to_status::DbErrorOptionToStatus;
use super::db_error_to_status::DbErrorToStatus;
use super::rocks_db_accessor::RocksDbAccessor;
use super::source_sink::effects_sink::EffectsSink;
use super::table_value_server_trait::TableValueServerTrait;
use super::to_entity_trait::FromEntity;
use super::to_entity_trait::ToEntity;
use super::to_reference_trait::FromReference;
use super::to_reference_trait::ToReference;
use super::value_to_json::ValueToJson;
use crate::db::entity::index::DEFAULT_ID_FIELD;
use crate::db::entity::reference::table_value_reference::TableValueReference;
use crate::db::entity::reference::table_value_reference::TableValueReferenceTrait;
use crate::db::entity::table_value::TableValue;
use crate::ondo_remote;
use ondo_remote::*;
use serde_json::json;
use serde_json::Value;
use tonic::{Request, Response, Status};

impl ToReference<TableValueReference> for TableValueReferenceMessage {
    fn to_reference(&self) -> TableValueReference {
        TableValueReference {
            table_reference: self.table_reference.as_ref().unwrap().to_reference(),
            id: Value::json_to_value(&self.json_id),
        }
    }
}

impl ToReference<TableValueReference> for Request<TableValueReferenceMessage> {
    fn to_reference(&self) -> TableValueReference {
        self.get_ref().to_reference()
    }
}

impl ToReference<TableValueReference> for TableValueMessage {
    fn to_reference(&self) -> TableValueReference {
        let tr_msg = self.table_reference.as_ref().unwrap();
        let data = Value::json_to_value(&self.json_value);
        let useless_id = json!(0u64);
        let id = data.get(DEFAULT_ID_FIELD).unwrap_or(&useless_id);
        TableValueReference {
            table_reference: tr_msg.to_reference(),
            id: id.clone(),
        }
    }
}

impl ToReference<TableValueReference> for Request<TableValueMessage> {
    fn to_reference(&self) -> TableValueReference {
        self.get_ref().to_reference()
    }
}

impl ToEntity<Value> for TableValueMessage {
    fn to_entity(&self) -> TableValue {
        Value::json_to_value(&self.json_value)
    }
}

impl ToEntity<Value> for Request<TableValueMessage> {
    fn to_entity(&self) -> Value {
        self.get_ref().to_entity()
    }
}

impl FromReference<TableValueReference> for TableValueReferenceMessage {
    fn from_reference(r: TableValueReference) -> Self {
        TableValueReferenceMessage {
            table_reference: Some(TableReferenceMessage::from_reference(r.table_reference)),
            json_id: Value::value_to_json(&r.id),
        }
    }
}

impl FromEntity<Value> for JsonMessage {
    fn from_entity(entity: Value) -> Self {
        JsonMessage {
            json_value: Value::value_to_json(&entity),
        }
    }
}

impl FromEntity<Value> for Response<JsonMessage> {
    fn from_entity(entity: Value) -> Self {
        let msg = JsonMessage::from_entity(entity);
        Response::new(msg)
    }
}

impl TableValueServerTrait for RocksDbAccessor {
    fn create_value(&self, r: Request<TableValueMessage>) -> Result<Response<JsonMessage>, Status> {
        let mut reference = r.to_reference();
        let mut entity = r.to_entity();
        let (new_id, effects) = reference
            .post_table_value(&mut entity, self)
            .map_db_err_to_status()?;
        effects.apply_effects(self)?;
        let json_new_id = Value::value_to_json(&new_id);
        Ok(Response::new(JsonMessage {
            json_value: json_new_id,
        }))
    }

    fn delete_value(
        &self,
        r: Request<TableValueReferenceMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        r.to_reference()
            .delete_table_value()
            .map_db_err_to_status()?
            .apply_effects(self)
    }

    fn get_value(
        &self,
        r: Request<TableValueReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status> {
        r.to_reference()
            .get_table_value(self)
            .map_db_err_option_to_status()
            .map(|entity| Response::<JsonMessage>::from_entity(entity))
    }

    fn update_value(
        &self,
        r: Request<TableValueMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        r.to_reference()
            .put_table_value(&r.to_entity())
            .map_db_err_to_status()?
            .apply_effects(self)
    }
}
