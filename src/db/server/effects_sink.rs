use crate::db::entity::reference::effect::Effect;
use tonic::{Code, Status, Response};
use crate::ondo_remote::EmptyMessage;

pub(crate) trait EffectSink {
    fn apply_effects(&self) -> Result<Response<EmptyMessage>, Status>;
}

impl EffectSink for Vec<Effect> {

 fn apply_effects(&self) -> Result<Response<EmptyMessage>, Status> {
    // for effect in *self {
    //     match effect {
    //         Effect::CreateCf(_) => {}
    //         Effect::DeleteCf(_) => {}
    //     }
    // }
    todo!();
    Ok(Response::new(EmptyMessage {}))
}
}