use super::db_error_to_status::DbErrorOptionToStatus;
use super::db_error_to_status::DbErrorToStatus;
use super::domain_server_trait::DomainServerTrait;
use super::lockable_db::transaction_maker::LockableTransactionOrDb;
use super::source_sink::effects_sink::EffectsSink;
use crate::{
    db::{
        entity::domain::Domain,
        reference::domain_reference::{DomainReference, DomainReferenceTrait},
    },
    ondo_remote::*,
};
use tonic::{Request, Response, Status};

impl<'a> Into<DomainReference> for &'a DomainReferenceMessage {
    fn into(self) -> DomainReference {
        DomainReference {
            domain_name: self.domain_name.clone(),
        }
    }
}

impl Into<DomainReferenceMessage> for DomainReference {
    fn into(self) -> DomainReferenceMessage {
        DomainReferenceMessage {
            domain_name: self.domain_name,
        }
    }
}

impl<'a> Into<Domain> for &'a DomainMessage {
    fn into(self) -> Domain {
        Domain {
            reference: self.domain_reference.as_ref().unwrap().into(),
        }
    }
}

impl Into<DomainMessage> for Domain {
    fn into(self) -> DomainMessage {
        let r_msg: DomainReferenceMessage = self.reference.into();
        DomainMessage {
            domain_reference: Some(r_msg),
        }
    }
}

impl<'a> DomainServerTrait for LockableTransactionOrDb<'a> {
    fn create_domain(&self, r: Request<DomainMessage>) -> Result<Response<EmptyMessage>, Status> {
        let entity: Domain = r.get_ref().into();
        entity
            .reference
            .post_domain(&entity, self, self)
            .map_db_err_to_status()?
            .apply_effects(self)
    }

    fn delete_domain(
        &self,
        r: Request<DomainReferenceMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        let reference: DomainReference = r.get_ref().into();
        reference
            .delete_domain(self, self, self)
            .map_db_err_to_status()?
            .apply_effects(self)
    }

    fn get_domain(
        &self,
        r: Request<DomainReferenceMessage>,
    ) -> Result<Response<DomainMessage>, Status> {
        let reference: DomainReference = r.get_ref().into();
        reference
            .get_domain(self)
            .map_db_err_option_to_status()
            .map(|entity| Response::new(entity.into()))
    }

    fn update_domain(&self, r: Request<DomainMessage>) -> Result<Response<EmptyMessage>, Status> {
        let entity: Domain = r.get_ref().into();
        entity
            .reference
            .put_domain(&entity, self)
            .map_db_err_to_status()?
            .apply_effects(self)
    }

    fn list_tables(
        &self,
        r: Request<DomainReferenceMessage>,
    ) -> Result<Response<ArrayOfStringResponse>, Status> {
        let reference: DomainReference = r.get_ref().into();
        let names = reference.list_table_names(self).map_db_err_to_status()?;
        let response = ArrayOfStringResponse { values: names };
        Ok(Response::new(response))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_reference_message_into_domain_reference() {
        let message = DomainReferenceMessage {
            domain_name: "example.com".to_string(),
        };
        let reference: DomainReference = (&message).into();
        assert_eq!(reference.domain_name, "example.com");
    }

    #[test]
    fn test_domain_reference_into_domain_reference_message() {
        let reference = DomainReference {
            domain_name: "example.com".to_string(),
        };
        let message: DomainReferenceMessage = reference.into();
        assert_eq!(message.domain_name, "example.com");
    }

    #[test]
    fn test_domain_message_into_domain() {
        let reference = DomainReference {
            domain_name: "example.com".to_string(),
        };
        let message = DomainMessage {
            domain_reference: Some(reference.into()),
        };
        let domain: Domain = (&message).into();
        assert_eq!(domain.reference.domain_name, "example.com");
    }

    #[test]
    fn test_domain_into_domain_message() {
        let domain = Domain {
            reference: DomainReference {
                domain_name: "example.com".to_string(),
            },
        };
        let message: DomainMessage = domain.into();
        assert_eq!(message.domain_reference.unwrap().domain_name, "example.com");
    }
}
