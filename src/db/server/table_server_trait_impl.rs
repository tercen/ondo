use super::rocks_db_accessor::RocksDbAccessor;
use super::table_server_trait::TableServerTrait;
use crate::ondo_remote;
use ondo_remote::*;
use tonic::{Request, Response, Status};

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
