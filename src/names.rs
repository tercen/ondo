use std::fmt;

pub type Value = serde_json::Value;
pub type Key = Vec<Value>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainName(pub String);
impl DomainName {
    pub fn new(name: &str) -> DomainName {
        DomainName(name.to_string())
    }
}
impl fmt::Display for DomainName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableName(pub String);
impl TableName {
    pub fn new(name: &str) -> TableName {
        TableName(name.to_string())
    }
}
impl fmt::Display for TableName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexName(pub String); 
impl IndexName {
    pub fn new(name: &str) -> IndexName {
        IndexName(name.to_string())
    }
}
impl fmt::Display for IndexName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
