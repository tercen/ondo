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
use crate::db::server::lockable_db::transaction_maker::LockableTransactionOrDb;

#[derive(Clone)]
pub struct MyServer {
}

impl Default for MyServer {
    fn default() -> Self {
        MyServer {
        }
    }
}

impl MyServer {
    pub(crate) fn empty_message_sub_server(&self) -> EmptyMessageSubServer {
        EmptyMessageSubServer {}
    }

    pub(crate) fn version_sub_server(&self, lockable_db: LockableTransactionOrDb) -> VersionSubServer {
        VersionSubServer {
            lockable_db: lockable_db.clone(),
        }
    }

    pub(crate) fn database_server_ops_sub_server(&self, lockable_db: LockableTransactionOrDb) -> DatabaseServerOpsSubServer {
        DatabaseServerOpsSubServer {
            lockable_db: lockable_db.clone(),
        }
    }

    pub(crate) fn domain_ops_sub_server(&self, lockable_db: LockableTransactionOrDb) -> DomainOpsSubServer {
        DomainOpsSubServer {
            lockable_db: lockable_db.clone(),
        }
    }

    pub(crate) fn table_ops_sub_server(&self, lockable_db: LockableTransactionOrDb) -> TableOpsSubServer {
        TableOpsSubServer {
            lockable_db: lockable_db.clone(),
        }
    }

    pub(crate) fn table_value_ops_sub_server(&self, lockable_db: LockableTransactionOrDb) -> TableValueOpsSubServer {
        TableValueOpsSubServer {
            lockable_db: lockable_db.clone(),
        }
    }

    pub(crate) fn index_ops_sub_server(&self, lockable_db: LockableTransactionOrDb) -> IndexOpsSubServer {
        IndexOpsSubServer {
            lockable_db: lockable_db.clone(),
        }
    }

    pub(crate) fn indexed_value_ops_sub_server(&self, lockable_db: LockableTransactionOrDb) -> IndexedValueOpsSubServer {
        IndexedValueOpsSubServer {
            lockable_db: lockable_db.clone(),
        }
    }

    pub(crate) fn key_prefix_ops_sub_server(&self, lockable_db: LockableTransactionOrDb) -> KeyPrefixOpsSubServer {
        KeyPrefixOpsSubServer {
            lockable_db: lockable_db.clone(),
        }
    }

    pub(crate) fn text_index_ops_sub_server(&self, lockable_db: LockableTransactionOrDb) -> TextIndexOpsSubServer {
        TextIndexOpsSubServer {
            lockable_db: lockable_db.clone(),
        }
    }
}
