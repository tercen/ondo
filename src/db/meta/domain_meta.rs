use crate::db::names::*;

pub struct DomainMeta {
    pub name: DomainName,
    pub table_names: Vec<TableName>,
}
