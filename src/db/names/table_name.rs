// #![allow(dead_code)]
use std::fmt;

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
