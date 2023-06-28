use super::ondo_serializer::OndoSerializer;
use crate::db::entity::OndoKey;
use crate::db::reference::effect::TableValueEffect;

use crate::db::DbError;
use crate::db::server::lockable_db::transaction_or_db::TransactionOrDb;
use serde_json::Value;

pub(super) fn apply_effect<'a>(
    db: &TransactionOrDb<'a>,
    effect: &TableValueEffect,
) -> Result<(), DbError> {
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
