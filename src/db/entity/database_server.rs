use std::collections::HashMap;

pub struct DatabaseServerU;

pub struct DatabaseServer {
    pub databse_server_u: DatabaseServerU,
    pub domains: HashMap<String, ()>
}
