pub type Value = serde_json::Value;
pub type Key = Vec<Value>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainName(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableName(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexName(pub String); 
