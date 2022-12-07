use crate::{index_definition::IndexDefinition, types::*};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Query {
    PostTable(DomainName, TableName),
    DeleteTable(DomainName, TableName),
    PostIndex(DomainName, IndexDefinition),
    DeleteIndex(DomainName, TableName, IndexName),
    Get(DomainName, TableName, Key),
    Find(DomainName, TableName, IndexName, Key),
    Put(DomainName, TableName, Key, Value),
    Post(DomainName, TableName, Key, Value),
    Delete(DomainName, TableName, Key),
}
