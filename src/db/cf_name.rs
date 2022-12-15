use super::entity::DomainReference;
use super::entity::TableReference;
use super::entity::IndexReference;

pub struct CfName;

impl CfName {
    pub fn for_server_meta() -> String {
        //table names etc
        "/server/_".to_string()
    }

    pub fn for_domain_meta() -> String {
        //table names etc
        "/domains/_".to_string()
    }

    pub fn for_table_meta(r: &DomainReference) -> String {
        //indexes etc
        format!("/domains/{}/tables/_", r.domain_name)
    }

    pub fn for_table_values(r: &TableReference) -> String {
        //value rows
        format!("{}::/{}/_", r.domain_name, r.table_name)
    }

    pub fn for_index_values(r: &IndexReference) -> String {
        format!("{}::/{}/indexes/{}/_", r.domain_name, r.table_name, r.index_name)
    }
}


#[cfg(test)]
mod tests {
    // use super::*;

   
}
