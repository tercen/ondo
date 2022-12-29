use super::DbResult;
pub trait LowLevelRequests {
    fn get_from_cf(&self, cf_name: &str, key: &[u8]) -> DbResult<Option<Vec<u8>>>;
}