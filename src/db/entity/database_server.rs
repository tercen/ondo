use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DatabaseServer;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DatabaseServerStored {
    pub database_server: DatabaseServer,
    pub domains: HashMap<String, ()>
}
