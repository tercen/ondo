use super::ondo_serializer::OndoSerializer;
use crate::db::db_error::DbError;
use crate::db::entity::reference::effect::table_value_effect::TableValueEffect;
use crate::db::server::rocks_db_accessor::RocksDbAccessor;
use serde_json::Value;

pub(super) fn apply_effect(ra: &RocksDbAccessor, effect: &TableValueEffect) -> Result<(), DbError> {
    let guarded_db = ra.guarded_db();
    let db = RocksDbAccessor::db_read_lock(&guarded_db)?;
    match effect {
        TableValueEffect::Put(cf_name, key, value) => {
            let ondo_key = Value::ondo_serialize(&key.id)?;
            let ondo_value = Value::ondo_serialize(&value)?;
            let cf = db.cf_handle(&cf_name).ok_or(DbError::CfNotFound)?;
            // println!("DEBUG: Writing table value with key: {:?}", ondo_key);
            db.put_cf(&cf, ondo_key, ondo_value)
                .map_err(|err| DbError::RocksDbError(err))
        }
        TableValueEffect::Delete(cf_name, key) => {
            let ondo_key = Value::ondo_serialize(&key.id)?;
            let cf = db.cf_handle(&cf_name).ok_or(DbError::CfNotFound)?;
            db.delete_cf(&cf, ondo_key)
                .map_err(|err| DbError::RocksDbError(err))
        }
    }
}
