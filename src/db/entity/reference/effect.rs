//effect.rs
use super::database_server_reference::stored::DatabaseServerStoredEffect;
use super::domain_reference::stored::DomainStoredEffect;
use super::table_reference::stored::TableStoredEffect;
use super::index_value_reference::IndexValueEffect;
use super::table_value_reference::TableValueEffect;
use super::column_value_reference::ColumnValueEffect;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Effect {
    CreateCf(String),
    DeleteCf(String),
    DatabaseServerStoredEffect(DatabaseServerStoredEffect),
    DomainStoredEffect(DomainStoredEffect),
    TableStoredEffect(TableStoredEffect),
    IndexValueEffect(IndexValueEffect),
    TableValueEffect(TableValueEffect),
    ColumnValueEffect(ColumnValueEffect),
}

pub type Effects = Vec<Effect>;