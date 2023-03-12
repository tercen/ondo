//effect.rs
pub(crate) mod column_value_effect;
pub(crate) mod database_server_stored_effect;
pub(crate) mod domain_stored_effect;
pub(crate) mod index_value_effect;
pub(crate) mod table_stored_effect;
pub(crate) mod table_value_effect;

use column_value_effect::ColumnValueEffect;
use database_server_stored_effect::DatabaseServerStoredEffect;
use domain_stored_effect::DomainStoredEffect;
use index_value_effect::IndexValueEffect;
use table_stored_effect::TableStoredEffect;
use table_value_effect::TableValueEffect;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Effect {
    CreateCf(String),
    DeleteCf(String),
    DatabaseServerStoredEffect(DatabaseServerStoredEffect),
    DomainStoredEffect(DomainStoredEffect),
    TableStoredEffect(TableStoredEffect),
    IndexValueEffect(IndexValueEffect),
    TableValueEffect(TableValueEffect),
    ColumnValueEffect(ColumnValueEffect),
}

pub(crate) type Effects = Vec<Effect>;
