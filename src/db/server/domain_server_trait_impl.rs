use super::db_error_to_status::DbErrorOptionToStatus;
use super::db_error_to_status::DbErrorToStatus;
use super::domain_server_trait::DomainServerTrait;
use super::rocks_db_accessor::RocksDbAccessor;
use super::source_sink::effects_sink::EffectsSink;
use super::to_entity_trait::FromEntity;
use super::to_entity_trait::ToEntity;
use super::to_reference_trait::FromReference;
use super::to_reference_trait::ToReference;
use crate::db::entity::domain::Domain;
use crate::db::entity::reference::domain_reference::DomainReference;
use crate::db::entity::reference::domain_reference::DomainReferenceTrait;
use crate::ondo_remote::*;
use tonic::{Request, Response, Status};

impl ToReference<DomainReference> for DomainReferenceMessage {
    fn to_reference(&self) -> DomainReference {
        DomainReference {
            domain_name: self.domain_name.clone(),
        }
    }
}

impl ToReference<DomainReference> for Request<DomainReferenceMessage> {
    fn to_reference(&self) -> DomainReference {
        self.get_ref().to_reference()
    }
}

impl ToReference<DomainReference> for DomainMessage {
    fn to_reference(&self) -> DomainReference {
        let r_msg = self.domain_reference.as_ref().unwrap();
        r_msg.to_reference()
    }
}

impl ToReference<DomainReference> for Request<DomainMessage> {
    fn to_reference(&self) -> DomainReference {
        self.get_ref().to_reference()
    }
}

impl ToEntity<Domain> for DomainMessage {
    fn to_entity(&self) -> Domain {
        let r = self.to_reference();
        Domain { id: r }
    }
}

impl ToEntity<Domain> for Request<DomainMessage> {
    fn to_entity(&self) -> Domain {
        self.get_ref().to_entity()
    }
}

impl FromReference<DomainReference> for DomainReferenceMessage {
    fn from_reference(r: DomainReference) -> Self {
        DomainReferenceMessage {
            domain_name: r.domain_name,
        }
    }
}

impl FromEntity<Domain> for DomainMessage {
    fn from_entity(entity: Domain) -> Self {
        let r_msg = DomainReferenceMessage::from_reference(entity.id);
        DomainMessage {
            domain_reference: Some(r_msg),
        }
    }
}

impl FromEntity<Domain> for Response<DomainMessage> {
    fn from_entity(entity: Domain) -> Self {
        let msg = DomainMessage::from_entity(entity);
        Response::new(msg)
    }
}

impl DomainServerTrait for RocksDbAccessor {
    fn create_domain(&self, r: Request<DomainMessage>) -> Result<Response<EmptyMessage>, Status> {
        r.to_reference()
            .post_domain(&r.to_entity(), self, self)
            .map_db_err_to_status()?
            .apply_effects(self)
    }

    fn delete_domain(
        &self,
        r: Request<DomainReferenceMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        r.to_reference()
            .delete_domain(self, self, self)
            .map_db_err_to_status()?
            .apply_effects(self)
    }

    fn get_domain(
        &self,
        r: Request<DomainReferenceMessage>,
    ) -> Result<Response<DomainMessage>, Status> {
        r.to_reference()
            .get_domain(self)
            .map_db_err_option_to_status()
            .map(|entity| Response::<DomainMessage>::from_entity(entity))
    }

    fn update_domain(&self, r: Request<DomainMessage>) -> Result<Response<EmptyMessage>, Status> {
        r.to_reference()
            .put_domain(&r.to_entity(), self)
            .map_db_err_to_status()?
            .apply_effects(self)
    }

    fn list_tables(
        &self,
        r: Request<DomainReferenceMessage>,
    ) -> Result<Response<ArrayOfStringResponse>, Status> {
        let names = r
            .to_reference()
            .list_table_names(self)
            .map_db_err_to_status()?;
        let response = ArrayOfStringResponse { values: names };
        Ok(Response::new(response))
    }
}
