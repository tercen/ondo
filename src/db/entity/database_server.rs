use std::collections::HashMap;

pub struct DatabaseServer;

pub struct DatabaseServerStored {
    pub databse_server: DatabaseServer,
    pub domains: HashMap<String, ()>
}
