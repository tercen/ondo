//database_server.rs
use std::collections::HashMap;
// TODO: Version number of Database server

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DatabaseServer;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DatabaseServerStored {
    pub database_server: DatabaseServer,
    pub domains: HashMap<String, ()>
}
