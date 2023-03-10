use super::table_value_server_trait::TableValueServerTrait;
use super::rocks_db_accessor::RocksDbAccessor;
use tonic::{Request, Response, Status};

use crate::remote::*;

impl TableValueServerTrait for RocksDbAccessor {
    fn create_value(
        &self,
        _: Request<TableValueMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        todo!()
    }

    fn delete_value(
        &self,
        _: Request<TableValueReferenceMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        todo!()
    }

    fn get_value(
        &self,
        _: Request<TableValueReferenceMessage>,
    ) -> Result<Response<JsonResponse>, Status> {
        todo!()
    }

    fn update_value(
        &self,
        _: Request<TableValueMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        todo!()
    }
}
