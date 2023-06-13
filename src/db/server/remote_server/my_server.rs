use super::database_server_ops_sub_server::DatabaseServerOpsSubServer;
use super::domain_ops_sub_server::DomainOpsSubServer;
use super::empty_message_sub_server::EmptyMessageSubServer;
use super::index_ops_sub_server::IndexOpsSubServer;
use super::indexed_value_ops_sub_server::IndexedValueOpsSubServer;
use super::key_prefix_ops_sub_server::KeyPrefixOpsSubServer;
use super::table_ops_sub_server::TableOpsSubServer;
use super::table_value_ops_sub_server::TableValueOpsSubServer;
use super::text_index_ops_sub_server::TextIndexOpsSubServer;
use super::version_sub_server::VersionSubServer;
use crate::db::server::lockable_db::transaction_maker::TransactionMaker;
use crate::db::server::lockable_db::LOCKABLE_DB;

// FIXME: Meta RWLock is required to prevent meta operations during transactions

#[derive(Clone)]
pub struct MyServer {
    lockable_db: TransactionMaker<'static>,
}

impl Default for MyServer {
    fn default() -> Self {
        MyServer {
            lockable_db: TransactionMaker::new(LOCKABLE_DB.clone()),
        }
    }
}

impl MyServer {
    pub(crate) fn empty_message_sub_server(&self) -> EmptyMessageSubServer {
        EmptyMessageSubServer {}
    }

    pub(crate) fn version_sub_server(&self) -> VersionSubServer {
        VersionSubServer {
            lockable_db: self.lockable_db.clone(),
        }
    }

    pub(crate) fn database_server_ops_sub_server(&self) -> DatabaseServerOpsSubServer {
        DatabaseServerOpsSubServer {
            lockable_db: self.lockable_db.clone(),
        }
    }

    pub(crate) fn domain_ops_sub_server(&self) -> DomainOpsSubServer {
        DomainOpsSubServer {
            lockable_db: self.lockable_db.clone(),
        }
    }

    pub(crate) fn table_ops_sub_server(&self) -> TableOpsSubServer {
        TableOpsSubServer {
            lockable_db: self.lockable_db.clone(),
        }
    }

    pub(crate) fn table_value_ops_sub_server(&self) -> TableValueOpsSubServer {
        TableValueOpsSubServer {
            lockable_db: self.lockable_db.clone(),
        }
    }

    pub(crate) fn index_ops_sub_server(&self) -> IndexOpsSubServer {
        IndexOpsSubServer {
            lockable_db: self.lockable_db.clone(),
        }
    }

    pub(crate) fn indexed_value_ops_sub_server(&self) -> IndexedValueOpsSubServer {
        IndexedValueOpsSubServer {
            lockable_db: self.lockable_db.clone(),
        }
    }

    pub(crate) fn key_prefix_ops_sub_server(&self) -> KeyPrefixOpsSubServer {
        KeyPrefixOpsSubServer {
            lockable_db: self.lockable_db.clone(),
        }
    }

    pub(crate) fn text_index_ops_sub_server(&self) -> TextIndexOpsSubServer {
        TextIndexOpsSubServer {
            lockable_db: self.lockable_db.clone(),
        }
    }
}
