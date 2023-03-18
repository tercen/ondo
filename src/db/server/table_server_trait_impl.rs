use super::db_error_to_status::DbErrorOptionToStatus;
use super::db_error_to_status::DbErrorToStatus;
use super::rocks_db_accessor::RocksDbAccessor;
use super::source_sink::effects_sink::EffectsSink;
use super::table_server_trait::TableServerTrait;
use super::to_entity_trait::FromEntity;
use super::to_entity_trait::ToEntity;
use super::to_reference_trait::FromReference;
use super::to_reference_trait::ToReference;
use crate::db::entity::reference::table_reference::TableReference;
use crate::db::entity::reference::table_reference::TableReferenceTrait;
use crate::db::entity::table::Table;
use crate::ondo_remote;
use ondo_remote::*;
use tonic::{Request, Response, Status};

impl ToReference<TableReference> for TableReferenceMessage {
    fn to_reference(&self) -> TableReference {
        TableReference {
            domain_name: self.domain_reference.as_ref().unwrap().domain_name.clone(),
            table_name: self.table_name.clone(),
        }
    }
}

impl ToReference<TableReference> for Request<TableReferenceMessage> {
    fn to_reference(&self) -> TableReference {
        self.get_ref().to_reference()
    }
}

impl ToReference<TableReference> for TableMessage {
    fn to_reference(&self) -> TableReference {
        let r_msg = self.table_reference.as_ref().unwrap();
        r_msg.to_reference()
    }
}

impl ToReference<TableReference> for Request<TableMessage> {
    fn to_reference(&self) -> TableReference {
        self.get_ref().to_reference()
    }
}

impl ToEntity<Table> for TableMessage {
    fn to_entity(&self) -> Table {
        let r = self.to_reference();
        Table { id: r }
    }
}

impl ToEntity<Table> for Request<TableMessage> {
    fn to_entity(&self) -> Table {
        self.get_ref().to_entity()
    }
}

impl FromReference<TableReference> for TableReferenceMessage {
    fn from_reference(r: TableReference) -> Self {
        TableReferenceMessage {
            domain_reference: Some(DomainReferenceMessage {
                domain_name: r.domain_name,
            }),
            table_name: r.table_name,
        }
    }
}

impl FromEntity<Table> for TableMessage {
    fn from_entity(entity: Table) -> Self {
        let r_msg = TableReferenceMessage::from_reference(entity.id);
        TableMessage {
            table_reference: Some(r_msg),
        }
    }
}

impl FromEntity<Table> for Response<TableMessage> {
    fn from_entity(entity: Table) -> Self {
        let msg = TableMessage::from_entity(entity);
        Response::new(msg)
    }
}

impl TableServerTrait for RocksDbAccessor {
    fn create_table(&self, r: Request<TableMessage>) -> Result<Response<EmptyMessage>, Status> {
        r.to_reference()
            .post_table(&r.to_entity(), self, self)
            .map_db_err_to_status()?
            .apply_effects(self)
    }

    fn delete_table(
        &self,
        r: Request<TableReferenceMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        r.to_reference()
            .delete_table(self, self)
            .map_db_err_to_status()?
            .apply_effects(self)
    }

    fn get_table(
        &self,
        r: Request<TableReferenceMessage>,
    ) -> Result<Response<TableMessage>, Status> {
        r.to_reference()
            .get_table(self)
            .map_db_err_option_to_status()
            .map(|entity| Response::<TableMessage>::from_entity(entity))
    }

    fn update_table(&self, r: Request<TableMessage>) -> Result<Response<EmptyMessage>, Status> {
        r.to_reference()
            .put_table(&r.to_entity(), self)
            .map_db_err_to_status()?
            .apply_effects(self)
    }

    fn list_indexes(
        &self,
        r: Request<TableReferenceMessage>,
    ) -> Result<Response<ArrayOfStringResponse>, Status> {
        let names = r
            .to_reference()
            .list_index_names(self)
            .map_db_err_to_status()?;
        let response = ArrayOfStringResponse { values: names };
        Ok(Response::new(response))
    }

    fn list_values(
        &self,
        _: Request<TableReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status> {
        todo!()
    }

    fn list_values_by_id_range(
        &self,
        _: Request<TableIdRangeReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status> {
        todo!()
    }

    fn list_values_by_id_list(
        &self,
        _: Request<TableIdListReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status> {
        todo!()
    }
}
