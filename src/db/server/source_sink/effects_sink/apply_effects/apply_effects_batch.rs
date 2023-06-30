use crate::db::{
    db_error::DbError, reference::Effect, server::lockable_db::transaction_or_db::TransactionOrDb,
};

use super::{
    apply_effects_batch_db::apply_effects_batch_db,
    apply_effects_batch_transaction_or_db::apply_effects_batch_transaction_or_db,
};

pub(super) fn apply_effects_batch<'a>(
    transaction_or_db: &TransactionOrDb<'a>,
    effects: &[Effect],
) -> Result<(), DbError> {
    match transaction_or_db {
        TransactionOrDb::Transaction(_, _) => {
            apply_effects_batch_transaction_or_db(transaction_or_db, effects)
        }
        TransactionOrDb::Db(db) => apply_effects_batch_db(db, effects),
    }
}
