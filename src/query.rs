use crate::{index_definition::IndexDefinition, types::*};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Query {
    CreateTable(DomainName, TableName),
    DeleteTabel(DomainName, TableName),
    CreateIndex(DomainName, IndexDefinition),
    DeleteIndex(DomainName, TableName, IndexName),
    Get(DomainName, TableName, Key),
    Delete(DomainName, TableName, Key),
    Find(DomainName, TableName, IndexName, Key)
}
