use crate::db::{db_error::DbError, reference::Effect};
use rocksdb::{TransactionDB, WriteBatchWithTransaction};

use super::make_access_effect_batch::make_access_effect_batch;

pub(super) fn apply_effects_batch_db(
    db: &TransactionDB,
    effects: &[Effect],
) -> Result<(), DbError> {
    let mut batch = WriteBatchWithTransaction::default();

    for effect in effects {
        match effect {
            Effect::Access(access) => {
                make_access_effect_batch(db, access, &mut batch)?;
            }
            _ => unreachable!(),
        }
    }

    db
        .write(batch)
        .map_err(|e| DbError::TantivyError(e.to_string()))?;

    Ok(())
}
