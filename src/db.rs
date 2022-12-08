use crate::{
    index_definition::IndexDefinition, 
    request::RequestHandlers, 
    names::*,
};

#[derive(Debug, Clone, PartialEq, Eq)]
/// A command to be executed on a database server
pub enum DbCommand {
    ListDomains,
    CreateDomain(DomainName),
    DeleteDomain(DomainName),
    GetDomain(DomainName), //Metadata

    ListTables(DomainName),
    CreateTable(DomainName, TableName),
    DeleteTable(DomainName, TableName),
    GetTable(DomainName, TableName), //Metadata

    ListIndexes(DomainName, TableName),
    CreateIndex(DomainName, IndexDefinition),
    GetIndex(DomainName, TableName, IndexName), //Metadata
    DeleteIndex(DomainName, TableName, IndexName),

    ListValues(DomainName, TableName),
    CreateValue(DomainName, TableName, Key, Value),
    DeleteValue(DomainName, TableName, Key),
    GetValue(DomainName, TableName, Key),
    PutValue(DomainName, TableName, Key, Value),

    FindValues(DomainName, TableName, IndexName, Key),  //TODO: Replace Key with a key range for mango queries
}
    
pub struct DbServer {
    pub request_handlers: RequestHandlers,
}