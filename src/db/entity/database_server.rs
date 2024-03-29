//database_server.rs
use crate::db::reference::DatabaseServerReference;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize, Default)]
pub(crate) struct DatabaseServer {
    pub reference: DatabaseServerReference,
}

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub(crate) struct DatabaseServerStored {
    pub database_server: DatabaseServer,
    pub meta_revision: u64,
    pub domains: HashMap<String, ()>,
}
