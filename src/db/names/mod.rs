pub type Value = serde_json::Value;
pub type Key = Vec<Value>;

mod domain_name;
pub use domain_name::*;

mod table_name;
pub use table_name::*;

mod index_name;
pub use index_name::*;

mod cf_name;
pub use cf_name::*;

