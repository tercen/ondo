use super::ondo_serializer::OndoSerializer;
use crate::db::db_error::DbError;
use crate::db::entity::ondo_key::OndoKey;
use crate::db::entity::reference::effect::table_value_effect::TableValueEffect;
use crate::db::server::rocks_db_accessor::RocksDbAccessor;
use serde_json::Value;

pub(super) fn apply_effect(ra: &RocksDbAccessor, effect: &TableValueEffect) -> Result<(), DbError> {
    let guarded_db = ra.guarded_db();
    let db = RocksDbAccessor::db_read_lock(&guarded_db)?;
    match effect {
        TableValueEffect::Put(cf_name, ondo_key, value) => {
            let serialized_ondo_key = OndoKey::ondo_serialize(&ondo_key)?;
            let ondo_value = Value::ondo_serialize(&value)?;
            let cf = db.cf_handle(&cf_name).ok_or(DbError::CfNotFound)?;
            // println!("DEBUG: Writing table value with key: {:?}", ondo_key);
            db.put_cf(&cf, serialized_ondo_key, ondo_value)
                .map_err(|err| DbError::RocksDbError(err))
        }
        TableValueEffect::Delete(cf_name, ondo_key) => {
            let serialized_ondo_key = OndoKey::ondo_serialize(&ondo_key)?;
            let cf = db.cf_handle(&cf_name).ok_or(DbError::CfNotFound)?;
            db.delete_cf(&cf, serialized_ondo_key)
                .map_err(|err| DbError::RocksDbError(err))
        }
    }
}
