use super::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CfName(pub String);

impl CfName {
    pub fn for_db_meta() -> CfName {
        //table names etc
        CfName("/db_meta".to_string())
    }

    pub fn for_domain_meta() -> CfName {
        //table names etc
        CfName("/domain_meta/_".to_string())
    }

    pub fn for_table_meta(d: &DomainName) -> CfName {
        //indexes etc
        CfName(format!("{}::/table_meta/_", d.0))
    }

    pub fn for_table(d: &DomainName, t: &TableName) -> CfName {
        //value rows
        CfName(format!("{}::/{}/_", d.0, t.0))
    }

    pub fn for_index(d: &DomainName, t: &TableName, i: &IndexName) -> CfName {
        CfName(format!("{}::/{}/indexes/{}/_", d.0, t.0, i.0))
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
    fn test_cf_name_for_domain_meta() {
        let cf_name = CfName::for_domain_meta();
        assert_eq!(cf_name, CfName("/domain_meta/_".to_string()));
    }

    #[test]
    fn test_cf_name_for_table_meta() {
        let d = DomainName("sample_domain".to_string());
        let cf_name = CfName::for_table_meta(&d);
        assert_eq!(cf_name, CfName("sample_domain::/table_meta/_".to_string()));
    }

    #[test]
    fn test_cf_name_for_table() {
        let d = DomainName("sample_domain".to_string());
        let t = TableName("table".to_string());
        let cf_name = CfName::for_table(&d, &t);
        assert_eq!(cf_name, CfName("sample_domain::/table/_".to_string()));
    }

    #[test]
    fn test_cf_name_for_index() {
        let d = DomainName("sample_domain".to_string());
        let t = TableName("sample_table".to_string());
        let i = IndexName("sample_index".to_string());
        let cf_name = CfName::for_index(&d, &t, &i);
        assert_eq!(
            cf_name,
            CfName("sample_domain::/sample_table/indexes/sample_index/_".to_string())
        );
    }
}
