use crate::remote;
use remote::*;
use tonic::{Request, Response, Status};

pub trait TableServerTrait {
    fn create_table(&self, _: Request<TableMessage>) -> Result<Response<EmptyMessage>, Status>;
    fn delete_table(
        &self,
        _: Request<TableReferenceMessage>,
    ) -> Result<Response<EmptyMessage>, Status>;
    fn get_table(
        &self,
        _: Request<TableReferenceMessage>,
    ) -> Result<Response<TableMessage>, Status>;
    fn update_table(&self, _: Request<TableMessage>) -> Result<Response<EmptyMessage>, Status>;
    fn list_indexes(
        &self,
        _: Request<TableReferenceMessage>,
    ) -> Result<Response<ArrayOfStringResponse>, Status>;
    fn list_values(
        &self,
        _: Request<TableReferenceMessage>,
    ) -> Result<Response<JsonResponse>, Status>;
    fn list_values_by_id_range(
        &self,
        _: Request<TableIdRangeReferenceMessage>,
    ) -> Result<Response<JsonResponse>, Status>;
    fn list_values_by_id_list(
        &self,
        _: Request<TableIdListReferenceMessage>,
    ) -> Result<Response<JsonResponse>, Status>;
}
