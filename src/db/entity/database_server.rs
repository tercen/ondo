//database_server.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// TODO: Version number of Database server

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub(crate) struct DatabaseServer;

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub(crate) struct DatabaseServerStored {
    pub database_server: DatabaseServer,
    pub meta_revision: u64,
    pub domains: HashMap<String, ()>,
}
