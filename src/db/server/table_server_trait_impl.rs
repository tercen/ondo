use super::db_error_to_status::DbErrorOptionToStatus;
use super::db_error_to_status::DbErrorToStatus;
use super::lockable_db::transaction_maker::LockableTransactionOrDb;
use super::source_sink::effects_sink::EffectsSink;
use super::table_server_trait::TableServerTrait;
use crate::db::reference::requests::TableStoredIteratorRequests;
use crate::db::{
    entity::{table::Table, OndoKey, TableValue},
    reference::{
        table_reference::TableReference, TableReferenceTrait, TableValueReference,
        TableValueReferenceTrait,
    },
    DbError,
};
use crate::ondo_remote;
use ondo_remote::*;
use tonic::{Request, Response, Status};

impl<'a> Into<TableReference> for &'a TableReferenceMessage {
    fn into(self) -> TableReference {
        TableReference {
            domain_reference: self.domain_reference.as_ref().unwrap().into(),
            table_name: self.table_name.clone(),
        }
    }
}
impl Into<TableReference> for TableReferenceMessage {
    fn into(self) -> TableReference {
        let reference = &self;
        reference.into()
    }
}

impl<'a> Into<Table> for &'a TableMessage {
    fn into(self) -> Table {
        Table {
            reference: self.table_reference.as_ref().unwrap().into(),
        }
    }
}

impl Into<TableReferenceMessage> for TableReference {
    fn into(self) -> TableReferenceMessage {
        TableReferenceMessage {
            domain_reference: Some(self.domain_reference.into()),
            table_name: self.table_name,
        }
    }
}

impl Into<TableMessage> for Table {
    fn into(self) -> TableMessage {
        TableMessage {
            table_reference: Some(self.reference.into()),
        }
    }
}

struct TableIdRangeReference {
    table_reference: TableReference,
    start_key: OndoKey,
    end_key: OndoKey,
}
impl<'a> Into<TableIdRangeReference> for &'a TableIdRangeReferenceMessage {
    fn into(self) -> TableIdRangeReference {
        TableIdRangeReference {
            table_reference: self.table_reference.as_ref().unwrap().into(),
            start_key: self.start_key.as_ref().unwrap().into(),
            end_key: self.end_key.as_ref().unwrap().into(),
        }
    }
}

struct TableIdListReference {
    table_reference: TableReference,
    keys: Vec<OndoKey>,
}
impl<'a> Into<TableIdListReference> for &'a TableIdListReferenceMessage {
    fn into(self) -> TableIdListReference {
        TableIdListReference {
            table_reference: self.table_reference.as_ref().unwrap().into(),
            keys: self.keys.iter().map(|k| k.into()).collect(),
        }
    }
}

impl<'a> TableServerTrait for LockableTransactionOrDb<'a> {
    fn create_table(&self, r: Request<TableMessage>) -> Result<Response<EmptyMessage>, Status> {
        let entity: Table = r.get_ref().into();
        entity
            .reference
            .post_table(&entity, self, self)
            .map_db_err_to_status()?
            .apply_effects(self)
    }

    fn delete_table(
        &self,
        r: Request<TableReferenceMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        let reference: TableReference = r.get_ref().into();
        reference
            .delete_table(self, self)
            .map_db_err_to_status()?
            .apply_effects(self)
    }

    fn get_table(
        &self,
        r: Request<TableReferenceMessage>,
    ) -> Result<Response<TableMessage>, Status> {
        let reference: TableReference = r.get_ref().into();
        reference
            .get_table(self)
            .map_db_err_option_to_status()
            .map(|entity| Response::new(entity.into()))
    }

    fn update_table(&self, r: Request<TableMessage>) -> Result<Response<EmptyMessage>, Status> {
        let entity: Table = r.get_ref().into();
        entity
            .reference
            .put_table(&entity, self)
            .map_db_err_to_status()?
            .apply_effects(self)
    }

    fn list_indexes(
        &self,
        r: Request<TableReferenceMessage>,
    ) -> Result<Response<ArrayOfStringResponse>, Status> {
        let reference: TableReference = r.get_ref().into();
        let names = reference.list_index_names(self).map_db_err_to_status()?;
        let response = ArrayOfStringResponse { values: names };
        Ok(Response::new(response))
    }

    fn list_values(
        &self,
        r: Request<TableReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status> {
        let guard = self.read();
        let db = guard.inner();
        let table_stored_iterator_requests: &dyn TableStoredIteratorRequests<'_> = &db;
        let reference: TableReference = r.get_ref().into();
        let iterator = reference
            .all_values(table_stored_iterator_requests)
            .map_db_err_to_status()?;
        let values_result: Result<Vec<TableValue>, DbError> = iterator.collect();
        let values = values_result.map_db_err_to_status()?;
        let json = serde_json::to_string(&values).map_err(|e| Status::internal(e.to_string()))?;
        let response = Response::new(JsonMessage { json });
        Ok(response)
    }

    fn list_values_by_key_prefix(
        &self,
        r: Request<TableValueReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status> {
        let guard = self.read();
        let db = guard.inner();
        let value_reference: TableValueReference = r.get_ref().into();
        let reference = value_reference.table_reference;
        let key_prefix = value_reference.id; // Assuming 'id' is the key_prefix field in TableValueReference
        let iterator = reference
            .all_values_with_key_prefix(key_prefix, &db)
            .map_db_err_to_status()?;
        let values_result: Result<Vec<TableValue>, DbError> = iterator.collect();
        let values = values_result.map_db_err_to_status()?;
        let json = serde_json::to_string(&values).map_err(|e| Status::internal(e.to_string()))?;
        let response = Response::new(JsonMessage { json });
        Ok(response)
    }

    fn list_values_by_id_range(
        &self,
        r: Request<TableIdRangeReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status> {
        let guard = self.read();
        let db = guard.inner();
        let range_reference: TableIdRangeReference = r.get_ref().into();
        let reference = range_reference.table_reference;
        let start_key = range_reference.start_key;
        let end_key = range_reference.end_key;
        let iterator = reference
            .all_values_with_key_range(start_key, end_key, &db) 
            .map_db_err_to_status()?;
        let values_result: Result<Vec<TableValue>, DbError> = iterator.collect();
        let values = values_result.map_db_err_to_status()?;
        let json = serde_json::to_string(&values).map_err(|e| Status::internal(e.to_string()))?;
        let response = Response::new(JsonMessage { json });
        Ok(response)
    }

    fn list_values_by_id_list(
        &self,
        r: Request<TableIdListReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status> {
        let table_id_list_reference: TableIdListReference = r.get_ref().into();
        let table_reference = table_id_list_reference.table_reference;
        let keys = table_id_list_reference.keys;

        let values_result: Result<Vec<TableValue>, DbError> = keys
            .into_iter()
            .map(|ondo_key| {
                let table_value_reference = TableValueReference {
                    table_reference: table_reference.clone(),
                    id: ondo_key,
                };
                table_value_reference
                    .get_table_value(self)
                    .and_then(|opt| opt.ok_or(DbError::NotFound))
            })
            .collect();

        let values = values_result.map_db_err_to_status()?;
        let json = serde_json::to_string(&values).map_err(|e| Status::internal(e.to_string()))?;
        let response = Response::new(JsonMessage { json });
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::reference::domain_reference::DomainReference;
    use crate::db::reference::table_reference::TableReference;

    #[test]
    fn test_table_reference_message_into_table_reference() {
        let message = TableReferenceMessage {
            domain_reference: Some(DomainReferenceMessage {
                domain_name: "example.com".to_string(),
            }),
            table_name: "table1".to_string(),
        };
        let reference: TableReference = (&message).into();
        assert_eq!(reference.domain_reference.domain_name, "example.com");
        assert_eq!(reference.table_name, "table1");
    }

    #[test]
    fn test_table_reference_into_table_reference_message() {
        let reference = TableReference {
            domain_reference: DomainReference {
                domain_name: "example.com".to_string(),
            },
            table_name: "table1".to_string(),
        };
        let message: TableReferenceMessage = reference.into();
        assert_eq!(message.domain_reference.unwrap().domain_name, "example.com");
        assert_eq!(message.table_name, "table1");
    }

    #[test]
    fn test_table_message_into_table() {
        let reference = TableReference {
            domain_reference: DomainReference {
                domain_name: "example.com".to_string(),
            },
            table_name: "table1".to_string(),
        };
        let message = TableMessage {
            table_reference: Some(reference.into()),
        };
        let table: Table = (&message).into();
        assert_eq!(table.reference.domain_reference.domain_name, "example.com");
        assert_eq!(table.reference.table_name, "table1");
    }

    #[test]
    fn test_table_into_table_message() {
        let table = Table {
            reference: TableReference {
                domain_reference: DomainReference {
                    domain_name: "example.com".to_string(),
                },
                table_name: "table1".to_string(),
            },
        };
        let message: TableMessage = table.into();
        assert_eq!(
            message
                .clone()
                .table_reference
                .unwrap()
                .domain_reference
                .unwrap()
                .domain_name,
            "example.com"
        );
        assert_eq!(message.table_reference.unwrap().table_name, "table1");
    }
}
