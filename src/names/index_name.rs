use std::fmt;

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
