// text_index_server_trait.rs
use crate::ondo_remote;
use ondo_remote::*;
use tonic::{Request, Response, Status};

use super::lockable_db::LockableDb;

pub trait TextIndexServerTrait {
    fn create_text_index(
        &self,
        r: Request<TextIndexMessage>,
    ) -> Result<Response<EmptyMessage>, Status>;

    fn delete_text_index(
        &self,
        r: Request<TextIndexReferenceMessage>,
    ) -> Result<Response<EmptyMessage>, Status>;

    fn get_text_index(
        &self,
        r: Request<TextIndexReferenceMessage>,
    ) -> Result<Response<TextIndexMessage>, Status>;

    fn update_text_index(
        &self,
        r: Request<TextIndexMessage>,
    ) -> Result<Response<EmptyMessage>, Status>;

    fn search_text_index(
        &self,
        r: Request<TantivyQueryMessage>,
        lockable_db: LockableDb,
    ) -> Result<Response<JsonMessage>, Status>;
}
