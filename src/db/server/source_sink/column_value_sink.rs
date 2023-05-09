use super::ondo_serializer::OndoSerializer;
use crate::db::entity::ondo_key::OndoKey;
use crate::db::reference::effect::ColumnValueEffect;
use crate::db::server::lockable_db::transaction_maker::TransactionMaker;
use crate::db::DbError;
use serde_json::Value;

pub(super) fn apply_effect(
    ra: &TransactionMaker,
    effect: &ColumnValueEffect,
) -> Result<(), DbError> {
    let db = ra.read();
    match effect {
        ColumnValueEffect::Put(cf_name, key, value) => {
            let ondo_key = OndoKey::ondo_serialize(&key)?;
            let ondo_value = Value::ondo_serialize(&value)?;
            let cf = db.cf_handle(&cf_name).ok_or(DbError::CfNotFound)?;
            db.put_cf(&cf, ondo_key, ondo_value)
                .map_err(|err| DbError::RocksDbError(err))
        }
        ColumnValueEffect::Delete(cf_name, key) => {
            let ondo_key = OndoKey::ondo_serialize(&key)?;
            let cf = db.cf_handle(&cf_name).ok_or(DbError::CfNotFound)?;
            db.delete_cf(&cf, ondo_key)
                .map_err(|err| DbError::RocksDbError(err))
        }
    }
}
