use std::collections::HashMap;

pub struct DatabaseServer;

pub struct DatabaseServerStored {
    pub database_server: DatabaseServer,
    pub domains: HashMap<String, ()>
}
