#![allow(dead_code)]
mod db_error;
mod effects;
mod entity;
mod requests;
pub use db_error::DbError;
pub use db_error::DbResult;
pub use effects::*;
pub use requests::*;

// #[derive(Debug, Clone, PartialEq, Eq)]
// A command to be executed on a database server
// pub enum DbCommand {
//     ListDomains,
//     CreateDomain(&str),
//     DeleteDomain(&str),
//     GetDomain(&str), //Metadata

//     ListTables(&str),
//     CreateTable(DomainName, TableName),
//     DeleteTable(DomainName, TableName),
//     GetTable(DomainName, TableName), //Metadata

//     ListIndexes(DomainName, TableName),
//     GetIndex(DomainName, TableName, IndexName), //Metadata
//     PutIndex(DomainName, Index),
//     DeleteIndex(DomainName, TableName, IndexName),

//     ListValues(DomainName, TableName),
//     CreateValue(DomainName, TableName, Key, Value),
//     DeleteValue(DomainName, TableName, Key),
//     GetValue(DomainName, TableName, Key),
//     PutValue(DomainName, TableName, Key, Value),

//     FindValues(DomainName, TableName, IndexName, Key),  //TODO: Replace Key with a key range for mango queries
// }
 

