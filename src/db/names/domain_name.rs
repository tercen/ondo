use std::fmt;

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
