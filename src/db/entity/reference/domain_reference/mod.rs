//domain_reference.rs
use super::{
    effect::{Effect, Effects},
    CfNameMaker, DatabaseServerReference,
};
use crate::db::entity::reference::requests::database_server_stored_requests::DatabaseServerStoredRequests;
use crate::db::entity::reference::requests::domain_stored_requests::DomainStoredRequests;
use crate::db::entity::reference::requests::table_stored_requests::TableStoredRequests;
use crate::db::{
    db_error::{DbError, DbResult},
    entity::{Domain, DomainStored},
};
use serde::{Deserialize, Serialize};

pub(crate) mod stored;
use stored::*;

pub(crate) trait DomainReferenceTrait {
    fn get_domain(&self, requests: &dyn DomainStoredRequests) -> DbResult<Option<Domain>>;
    fn put_domain(&self, domain: &Domain, requests: &dyn DomainStoredRequests)
        -> DbResult<Effects>;
    fn post_domain(
        &self,
        domain: &Domain,
        requests: &dyn DomainStoredRequests,
        parent_requests: &dyn DatabaseServerStoredRequests,
    ) -> DbResult<Effects>;
    fn delete_domain(
        &self,
        table_requests: &dyn TableStoredRequests,
        domain_requests: &dyn DomainStoredRequests,
        parent_requests: &dyn DatabaseServerStoredRequests,
    ) -> DbResult<Effects>;
    fn list_table_names(&self, requests: &dyn DomainStoredRequests) -> DbResult<Vec<String>>;
}

pub(crate) type DomainName = String;
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct DomainReference {
    pub domain_name: DomainName,
}

impl DomainReference {
    pub fn new(domain_name: &str) -> Self {
        DomainReference {
            domain_name: domain_name.to_string(),
        }
    }

    pub fn to_database_server_reference(&self) -> DatabaseServerReference {
        DatabaseServerReference::new()
    }
}

impl DomainReferenceTrait for DomainReference {
    fn get_domain(&self, requests: &dyn DomainStoredRequests) -> DbResult<Option<Domain>> {
        self.get_domain_stored(requests)
            .map(|opt| opt.map(|domain_stored| domain_stored.domain))
    }

    fn put_domain(
        &self,
        domain: &Domain,
        requests: &dyn DomainStoredRequests,
    ) -> DbResult<Effects> {
        let stored_opt = self.get_domain_stored(requests)?;
        let stored = stored_opt.ok_or(DbError::DomainNotInitialized)?;
        let mut new_stored = stored.clone();
        new_stored.domain = (*domain).clone();
        self.put_domain_stored(&new_stored)
    }

    fn post_domain(
        &self,
        domain: &Domain,
        requests: &dyn DomainStoredRequests,
        parent_requests: &dyn DatabaseServerStoredRequests,
    ) -> DbResult<Effects> {
        let stored_opt = self.get_domain_stored(requests)?;
        match stored_opt {
            Some(_) => {
                // Data already exists, return AlreadyExists error
                Err(DbError::AlreadyExists)
            }
            None => {
                let new_stored = DomainStored {
                    domain: (*domain).clone(),
                    tables: Default::default(),
                };
                self.post_domain_stored(&new_stored, parent_requests)
            }
        }
    }

    fn delete_domain(
        &self,
        table_requests: &dyn TableStoredRequests,
        domain_requests: &dyn DomainStoredRequests,
        parent_requests: &dyn DatabaseServerStoredRequests,
    ) -> DbResult<Effects> {
        self.delete_domain_stored(table_requests, domain_requests, parent_requests)
    }

    fn list_table_names(&self, requests: &dyn DomainStoredRequests) -> DbResult<Vec<String>> {
        self.list_table_names_(requests)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::entity::reference::database_server_reference::stored::tests::{
        create_database_server_stored, MockDatabaseServerStoredTestRequests,
    };
    use crate::db::entity::reference::domain_reference::stored::tests::{
        create_domain, create_domain_ref, create_domain_stored, MockDomainStoredTestRequests,
    };

    mod domain_reference_trait {
        use super::*;

        #[test]
        fn test_get_domain_failure() {
            let mut mock = MockDomainStoredTestRequests::new();
            let ref_trait = create_domain_ref();
            mock.expect_get_domain_stored().returning(|_, _| Ok(None));

            assert_eq!(
                ref_trait.get_domain(&mock),
                Ok(None),
                "get_domain should return Ok(None) if the key does not exist"
            );
        }

        #[test]
        fn test_put_domain_failure() {
            let mut mock = MockDomainStoredTestRequests::new();
            let ref_trait = create_domain_ref();
            let domain = create_domain();

            mock.expect_get_domain_stored()
                .returning(move |_, _| Ok(None));

            assert_eq!(
                ref_trait.put_domain(&domain, &mock).unwrap_err(),
                DbError::DomainNotInitialized,
                "put_domain should return DbError::DomainNotInitialized if the key is not found"
            );
        }

        #[test]
        fn test_post_domain_failure() {
            let mut mock = MockDomainStoredTestRequests::new();
            let ref_trait = create_domain_ref();
            let domain = create_domain();

            mock.expect_get_domain_stored()
                .returning(move |_, _| Ok(Some(create_domain_stored())));

            let mut parent_mock = MockDatabaseServerStoredTestRequests::new();
            parent_mock
                .expect_get_database_server_stored()
                .returning(move |_, _| Ok(Some(create_database_server_stored())));

            assert_eq!(
                ref_trait
                    .post_domain(&domain, &mock, &parent_mock)
                    .unwrap_err(),
                DbError::AlreadyExists,
                "post_domain should return DbError::AlreadyExists if the key already exists"
            );
        }
    }
}
