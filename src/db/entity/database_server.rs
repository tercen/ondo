use std::collections::HashMap;

pub struct DataBaseServer;

pub struct DataBaseServerStored {
    pub databse_server: DataBaseServer,
    pub domains: HashMap<String, ()>
}
