use crate::ondo_remote;
use ondo_remote::*;
use tonic::{Request, Response, Status};

pub trait TableServerTrait {
    fn create_table(&self, _: Request<TableMessage>) -> Result<Response<EmptyMessage>, Status>;
    fn delete_table(
        &self,
        r: Request<TableReferenceMessage>,
    ) -> Result<Response<EmptyMessage>, Status>;
    fn get_table(
        &self,
        r: Request<TableReferenceMessage>,
    ) -> Result<Response<TableMessage>, Status>;
    fn update_table(&self, _: Request<TableMessage>) -> Result<Response<EmptyMessage>, Status>;
    fn list_indexes(
        &self,
        r: Request<TableReferenceMessage>,
    ) -> Result<Response<ArrayOfStringResponse>, Status>;
    fn list_values(
        &self,
        r: Request<TableReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status>;
    fn list_values_by_key_prefix(
        &self,
        r: Request<TableValueReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status>;
    fn list_values_by_id_range(
        &self,
        r: Request<TableIdRangeReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status>;
    fn list_values_by_id_list(
        &self,
        r: Request<TableIdListReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status>;
}
