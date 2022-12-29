use super::DbResult;

pub trait LowLevelEffectsHandler {
    fn handle(&self, effects: &[LowLevelEffect]) -> DbResult<()>;    
}

pub enum LowLevelEffect {
    CreateCf(String),
    PutToCf(String, Vec<u8>, Vec<u8>),
    DeleteFromCf(String, Vec<u8>),
}

