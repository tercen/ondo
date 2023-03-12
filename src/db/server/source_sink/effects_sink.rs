use crate::db::entity::reference::effect::Effect;
use tonic::{Status, Response};
use crate::ondo_remote::EmptyMessage;

pub(in crate::db::server) trait EffectsSink {
    fn apply_effects(&self) -> Result<Response<EmptyMessage>, Status>;
}

impl EffectsSink for Vec<Effect> {

 fn apply_effects(&self) -> Result<Response<EmptyMessage>, Status> {
    self.into_iter().for_each(|effect| {
        match effect {
            Effect::CreateCf(_) => {todo!();}
            Effect::DeleteCf(_) => {todo!();}
            Effect::DatabaseServerStoredEffect(_) => {todo!();}
            Effect::DomainStoredEffect(_) => {todo!();}
            Effect::TableStoredEffect(_) => {todo!();}
            Effect::TableValueEffect(_) => {todo!();}
            Effect::IndexValueEffect(_) => {todo!();}
            Effect::ColumnValueEffect(_) => {todo!();}
        }
    });
    Ok(Response::new(EmptyMessage {}))
}
}