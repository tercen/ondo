//TODO: use an XPATH like format for cf names
use super::{DomainReference, IndexReference, TableReference};

pub(crate) struct CfNameMaker;
pub(crate) type CfName = String;

impl CfNameMaker {
    pub fn for_server_meta() -> String {
        "/server".to_owned()
    }

    pub fn for_domain_meta() -> String {
        "/domains".to_owned()
    }

    pub fn for_table_counters(r: &DomainReference) -> String {
        format!("/domains/{}/counters", r.domain_name)
    }

    pub fn for_table_meta(r: &DomainReference) -> String {
        format!("/domains/{}/tables", r.domain_name)
    }

    pub fn for_table_values(r: &TableReference) -> String {
        format!("{}::/{}", r.domain_reference.domain_name, r.table_name)
    }

    pub fn for_index_values(r: &IndexReference) -> String {
        format!(
            "{}::/{}/indexes/{}",
            r.table_reference.domain_reference.domain_name,
            r.table_reference.table_name,
            r.index_name
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_for_server_meta() {
        assert_eq!(CfNameMaker::for_server_meta(), "/server");
    }

    #[test]
    fn test_for_domain_meta() {
        assert_eq!(CfNameMaker::for_domain_meta(), "/domains");
    }

    #[test]
    fn test_for_table_meta() {
        let r = DomainReference::new("domain1");
        assert_eq!(CfNameMaker::for_table_meta(&r), "/domains/domain1/tables");
    }

    #[test]
    fn test_for_table_values() {
        let r = TableReference::new("domain1", "table1");
        assert_eq!(CfNameMaker::for_table_values(&r), "domain1::/table1");
    }

    #[test]
    fn test_for_index_values() {
        let r = IndexReference::new("domain1", "table1", "index1");
        assert_eq!(
            CfNameMaker::for_index_values(&r),
            "domain1::/table1/indexes/index1"
        );
    }
}
