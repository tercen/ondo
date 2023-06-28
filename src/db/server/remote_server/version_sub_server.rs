use crate::db::server::lockable_db::transaction_or_db::TransactionOrDb;
use crate::db::server::lockable_db::version::Version;
use crate::ondo_remote::{
    transaction_response::ResponseType, EmptyMessage, TransactionResponse, VersionResponse,
};
use tonic::Status;

pub(crate) struct VersionSubServer<'a> {
    pub transaction_or_db: TransactionOrDb<'a>,
}

impl<'a> VersionSubServer<'a> {
    pub fn process_request(
        &self,
        _tx: tokio::sync::mpsc::Sender<Result<TransactionResponse, Status>>,
        _request: EmptyMessage,
    ) -> ResponseType {
        let version = Version::new();
        let response_type = ResponseType::VersionResponse(version.into());
        response_type
    }
}

impl Into<VersionResponse> for Version {
    fn into(self) -> VersionResponse {
        VersionResponse {
            major: self.major,
            minor: self.minor,
            patch: self.patch,
            commit: self.commit,
            date: self.date,
            features: self.features,
        }
    }
}
