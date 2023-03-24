use super::db_error_to_status::DbErrorOptionToStatus;
use super::db_error_to_status::DbErrorToStatus;
use super::rocks_db_accessor::RocksDbAccessor;
use super::source_sink::effects_sink::EffectsSink;
use super::table_server_trait::TableServerTrait;
use crate::db::entity::reference::table_reference::TableReference;
use crate::db::entity::reference::table_reference::TableReferenceTrait;
use crate::db::entity::table::Table;
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

impl TableServerTrait for RocksDbAccessor {
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
        _: Request<TableReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status> {
        todo!("indexing")
    }

    fn list_values_by_id_range(
        &self,
        _: Request<TableIdRangeReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status> {
        todo!("indexing")
    }

    fn list_values_by_id_list(
        &self,
        _: Request<TableIdListReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status> {
        todo!("indexing")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::entity::reference::domain_reference::DomainReference;
    use crate::db::entity::reference::table_reference::TableReference;

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
        assert_eq!(message.clone().table_reference.unwrap().domain_reference.unwrap().domain_name, "example.com");
        assert_eq!(message.table_reference.unwrap().table_name, "table1");
    }
}
