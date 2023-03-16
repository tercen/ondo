use super::db_error_to_status::DbErrorOptionToStatus;
use super::db_error_to_status::DbErrorToStatus;
use super::rocks_db_accessor::RocksDbAccessor;
use super::source_sink::effects_sink::EffectsSink;
use super::table_server_trait::TableServerTrait;
use super::to_entity_trait::FromEntity;
use super::to_entity_trait::ToEntity;
use super::to_reference_trait::ToReference;
use crate::db::entity::reference::table_reference::TableReference;
use crate::db::entity::reference::table_reference::TableReferenceTrait;
use crate::db::entity::table::Table;
use crate::ondo_remote;
use ondo_remote::*;
use tonic::{Request, Response, Status};

// impl ToReference<TableReference> for Request<TableReferenceMessage> {
//     fn to_reference(&self) -> TableReference {
//         TableReference {
//             domain_name: self.get_ref().domain_name.clone(),
//         }
//     }
// }

// impl ToReference<TableReference> for Request<TableMessage> {
//     fn to_reference(&self) -> TableReference {
//         let msg = self.get_ref();
//         let r_msg = msg.domain_reference.as_ref().unwrap();
//         TableReference {
//             domain_name: r_msg.domain_name.clone(),
//         }
//     }
// }

// impl ToEntity<Table> for Request<TableMessage> {
//     fn to_entity(&self) -> Table {
//         let r = self.to_reference();
//         Table { id: r }
//     }
// }

// impl FromEntity<Table> for Response<TableMessage> {
//     fn from_entity(entity: Table) -> Self {
//         let r = entity.id;
//         let r_msg = TableReferenceMessage {
//             domain_name: r.domain_name,
//         };
//         Response::new(TableMessage {
//             domain_reference: Some(r_msg),
//         })
//     }
// }

impl TableServerTrait for RocksDbAccessor {
    fn create_table(&self, _: Request<TableMessage>) -> Result<Response<EmptyMessage>, Status> {
        todo!()
    }

    fn delete_table(
        &self,
        _: Request<TableReferenceMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        todo!()
    }

    fn get_table(
        &self,
        _: Request<TableReferenceMessage>,
    ) -> Result<Response<TableMessage>, Status> {
        todo!()
    }

    fn update_table(&self, _: Request<TableMessage>) -> Result<Response<EmptyMessage>, Status> {
        todo!()
    }

    fn list_indexes(
        &self,
        _: Request<TableReferenceMessage>,
    ) -> Result<Response<ArrayOfStringResponse>, Status> {
        todo!()
    }

    fn list_values(
        &self,
        _: Request<TableReferenceMessage>,
    ) -> Result<Response<JsonResponse>, Status> {
        todo!()
    }

    fn list_values_by_id_range(
        &self,
        _: Request<TableIdRangeReferenceMessage>,
    ) -> Result<Response<JsonResponse>, Status> {
        todo!()
    }

    fn list_values_by_id_list(
        &self,
        _: Request<TableIdListReferenceMessage>,
    ) -> Result<Response<JsonResponse>, Status> {
        todo!()
    }
}
