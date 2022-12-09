use super::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CfName(pub String);

impl CfName {
    pub fn for_db() -> CfName {  //domain rows
        CfName("db".to_string())
    }

    pub fn for_domain(d: &DomainName) -> CfName { //table rows
        CfName(format!("{}::", d.0))
    }

    pub fn for_table(d: &DomainName, t: &TableName) -> CfName { //value rows
        CfName(format!("{}::/{}", d.0, t.0))
    }

    pub fn for_index(d: &DomainName, t: &TableName, i: &IndexName) -> CfName {
        CfName(format!("{}::/{}/indexes/{}", d.0, t.0, i.0))
    }
}
impl fmt::Display for CfName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cf_name_for_db() {
        let cf_name = CfName::for_db();
        assert_eq!(cf_name, CfName("db".to_string()));
    }

    #[test]
    fn test_cf_name_for_domain() {
        let d = DomainName("domain".to_string());
        let cf_name = CfName::for_domain(&d);
        assert_eq!(cf_name, CfName("domain::".to_string()));
    }

    #[test]
    fn test_cf_name_for_table() {
        let d = DomainName("domain".to_string());
        let t = TableName("table".to_string());
        let cf_name = CfName::for_table(&d, &t);
        assert_eq!(cf_name, CfName("domain::/table".to_string()));
    }

    #[test]
    fn test_cf_name_for_index() {
        let d = DomainName("domain".to_string());
        let t = TableName("table".to_string());
        let i = IndexName("index".to_string());
        let cf_name = CfName::for_index(&d, &t, &i);
        assert_eq!(cf_name, CfName("domain::/table/indexes/index".to_string()));
    }
}