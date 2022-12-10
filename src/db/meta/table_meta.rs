use crate::db::names::*;

pub struct TableMeta {
    pub domain_name: DomainName,
    pub name: TableName,
    pub index_names: Vec<IndexName>,
}
