use std::collections::HashMap;

pub struct DataBaseServer;

pub struct DataBaseServerStored {
    pub databse_server_u: DataBaseServer,
    pub domains: HashMap<String, ()>
}
