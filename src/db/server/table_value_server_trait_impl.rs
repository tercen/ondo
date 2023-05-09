use super::db_error_to_status::DbErrorOptionToStatus;
use super::db_error_to_status::DbErrorToStatus;
use super::lockable_db::transaction_maker::TransactionMaker;
use super::source_sink::effects_sink::EffectsTasksSink;
use super::table_value_server_trait::TableValueServerTrait;
use crate::db::reference::{
    table_value_reference::{CreateTableValueReference, CreateTableValueReferenceTrait},
    TableValueReference, TableValueReferenceTrait,
};
use crate::ondo_remote;
use ondo_remote::*;
use serde_json::Value;
use tonic::{Request, Response, Status};

impl<'a> Into<TableValueReference> for &'a TableValueReferenceMessage {
    fn into(self) -> TableValueReference {
        TableValueReference {
            table_reference: self.table_reference.as_ref().unwrap().into(),
            id: self.key.as_ref().unwrap().into(),
        }
    }
}
impl<'a> Into<CreateTableValueReference> for &'a CreateTableValueReferenceMessage {
    fn into(self) -> CreateTableValueReference {
        CreateTableValueReference {
            table_reference: self.table_reference.as_ref().unwrap().into(),
            id: self.key.as_ref().unwrap().into(),
        }
    }
}

#[derive(Clone)]
struct TableValuePayload {
    table_reference: TableValueReference,
    value: Value,
}
impl<'a> Into<TableValuePayload> for &'a TableValueMessage {
    fn into(self) -> TableValuePayload {
        TableValuePayload {
            table_reference: self.table_value_reference.as_ref().unwrap().into(),
            value: serde_json::from_str(&self.json).unwrap(),
        }
    }
}

#[derive(Clone)]
struct CreateTableValuePayload {
    create_table_reference: CreateTableValueReference,
    value: Value,
}
impl<'a> Into<CreateTableValuePayload> for &'a CreateTableValueMessage {
    fn into(self) -> CreateTableValuePayload {
        CreateTableValuePayload {
            create_table_reference: self.create_table_value_reference.as_ref().unwrap().into(),
            value: serde_json::from_str(&self.json).unwrap(),
        }
    }
}

impl<'a> TableValueServerTrait for TransactionMaker<'a> {
    fn create_value(
        &self,
        r: Request<CreateTableValueMessage>,
    ) -> Result<Response<OndoKeyMessage>, Status> {
        let payload: CreateTableValuePayload = r.get_ref().into();
        let reference = payload.create_table_reference;
        let mut entity = payload.value;
        let (new_id, effects, tasks) = reference
            .post_table_value(&mut entity, self, self, self)
            .map_db_err_to_status()?;
        (effects, tasks).apply_effects_queue_tasks(self)?;
        Ok(Response::new(new_id.into()))
    }

    fn delete_value(
        &self,
        r: Request<TableValueReferenceMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        let reference: TableValueReference = r.get_ref().into();
        reference
            .delete_table_value(self, self)
            .map_db_err_to_status()?
            .apply_effects_queue_tasks(self)
    }

    fn get_value(
        &self,
        r: Request<TableValueReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status> {
        let reference: TableValueReference = r.get_ref().into();
        reference
            .get_table_value(self)
            .map_db_err_option_to_status()
            .map(|entity| Response::new(entity.into()))
    }

    fn update_value(
        &self,
        r: Request<TableValueMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        let payload: TableValuePayload = r.get_ref().into();
        let entity = payload.value;
        let reference = payload.table_reference;
        reference
            .put_table_value(&entity, self, self)
            .map_db_err_to_status()?
            .apply_effects_queue_tasks(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_value_reference_message_into_table_value_reference() {
        let message = TableValueReferenceMessage {
            table_reference: Some(TableReferenceMessage {
                domain_reference: Some(DomainReferenceMessage {
                    domain_name: "example.com".to_string(),
                }),
                table_name: "table1".to_string(),
            }),
            key: Some(OndoKeyMessage {
                json_keys: vec![r#"{"key":"value"}"#.to_string()],
            }),
        };
        let reference: TableValueReference = (&message).into();
        assert_eq!(
            reference.table_reference.domain_reference.domain_name,
            "example.com"
        );
        assert_eq!(reference.table_reference.table_name, "table1");
        assert_eq!(reference.id.values.len(), 1);
        assert_eq!(reference.id.values[0]["key"], "value");
    }

    #[test]
    fn test_table_value_message_into_table_value_payload() {
        let message = TableValueMessage {
            table_value_reference: Some(TableValueReferenceMessage {
                table_reference: Some(TableReferenceMessage {
                    domain_reference: Some(DomainReferenceMessage {
                        domain_name: "example.com".to_string(),
                    }),
                    table_name: "table1".to_string(),
                }),
                key: Some(OndoKeyMessage {
                    json_keys: vec![r#"{"key":"value"}"#.to_string()],
                }),
            }),
            json: r#"{"key":"value"}"#.to_string(),
        };
        let payload: TableValuePayload = (&message).into();
        assert_eq!(
            payload
                .table_reference
                .table_reference
                .domain_reference
                .domain_name,
            "example.com"
        );
        assert_eq!(payload.table_reference.table_reference.table_name, "table1");
        assert_eq!(payload.table_reference.id.values.len(), 1);
        assert_eq!(payload.table_reference.id.values[0]["key"], "value");
        assert_eq!(payload.value["key"], "value");
    }

    #[test]
    fn test_create_table_value_reference_message_into_create_table_value_reference() {
        let message = CreateTableValueReferenceMessage {
            table_reference: Some(TableReferenceMessage {
                domain_reference: Some(DomainReferenceMessage {
                    domain_name: "example.com".to_string(),
                }),
                table_name: "table1".to_string(),
            }),
            key: Some(OptionalOndoKeyMessage {
                ondo_key: Some(OndoKeyMessage {
                    json_keys: vec![r#"{"key":"value"}"#.to_string()],
                }),
            }),
        };
        let reference: CreateTableValueReference = (&message).into();
        assert_eq!(
            reference.table_reference.domain_reference.domain_name,
            "example.com"
        );
        assert_eq!(reference.table_reference.table_name, "table1");
        assert_eq!(reference.clone().id.unwrap().values.len(), 1);
        assert_eq!(reference.id.unwrap().values[0]["key"], "value");
    }

    #[test]
    fn test_create_table_value_message_into_create_table_value_payload() {
        let message = CreateTableValueMessage {
            create_table_value_reference: Some(CreateTableValueReferenceMessage {
                table_reference: Some(TableReferenceMessage {
                    domain_reference: Some(DomainReferenceMessage {
                        domain_name: "example.com".to_string(),
                    }),
                    table_name: "table1".to_string(),
                }),
                key: Some(OptionalOndoKeyMessage {
                    ondo_key: Some(OndoKeyMessage {
                        json_keys: vec![r#"{"key":"value"}"#.to_string()],
                    }),
                }),
            }),
            json: r#"{"key":"value"}"#.to_string(),
        };
        let payload: CreateTableValuePayload = (&message).into();
        assert_eq!(
            payload
                .create_table_reference
                .table_reference
                .domain_reference
                .domain_name,
            "example.com"
        );
        assert_eq!(
            payload.create_table_reference.table_reference.table_name,
            "table1"
        );
        assert_eq!(
            payload
                .clone()
                .create_table_reference
                .id
                .unwrap()
                .values
                .len(),
            1
        );
        assert_eq!(
            payload.create_table_reference.id.unwrap().values[0]["key"],
            "value"
        );
        assert_eq!(payload.value["key"], "value");
    }
}
