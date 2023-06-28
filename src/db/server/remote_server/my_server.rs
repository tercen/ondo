use rocksdb::TransactionDB;

use crate::db::server::lockable_db::transaction_or_db::TransactionOrDb;

use super::database_server_ops_sub_server::DatabaseServerOpsSubServer;
use super::domain_ops_sub_server::DomainOpsSubServer;
use super::empty_message_sub_server::EmptyMessageSubServer;
use super::index_ops_sub_server::IndexOpsSubServer;
use super::indexed_value_ops_sub_server::IndexedValueOpsSubServer;
use super::tabled_value_ops_sub_server::TabledValueOpsSubServer;
use super::table_ops_sub_server::TableOpsSubServer;
use super::table_value_ops_sub_server::TableValueOpsSubServer;
use super::text_index_ops_sub_server::TextIndexOpsSubServer;
use super::version_sub_server::VersionSubServer;

#[derive(Clone)]
pub struct MyServer {}

impl Default for MyServer {
    fn default() -> Self {
        MyServer {}
    }
}

impl<'a> MyServer { 
    pub(crate) fn empty_message_sub_server(&self) -> EmptyMessageSubServer {
        EmptyMessageSubServer {}
    }

    pub(crate) fn version_sub_server<'b>(
        &'a self,
        transaction_or_db: TransactionOrDb<'b>,
    ) -> VersionSubServer<'b> {
        VersionSubServer {
            transaction_or_db: transaction_or_db.clone(),
        }
    }

    pub(crate) fn database_server_ops_sub_server(
        &'a self,
        db: &'a mut TransactionDB,
    ) -> DatabaseServerOpsSubServer {
        DatabaseServerOpsSubServer {
            db,
        }
    }

    pub(crate) fn domain_ops_sub_server(
        &'a self,
        db: &'a mut TransactionDB,
    ) -> DomainOpsSubServer {
        DomainOpsSubServer {
            db,
        }
    }

    pub(crate) fn table_ops_sub_server(
        &'a self,
        db: &'a mut TransactionDB,
    ) -> TableOpsSubServer {
        TableOpsSubServer {
            db,
        }
    }

    pub(crate) fn table_value_ops_sub_server<'b>(
        &'a self,
        db: TransactionOrDb<'b>,
    ) -> TableValueOpsSubServer<'b> {
        TableValueOpsSubServer {
            transaction_or_db: db.clone(),
        }
    }

    pub(crate) fn index_ops_sub_server(
        &'a self,
        db: &'a mut TransactionDB,
    ) -> IndexOpsSubServer {
        IndexOpsSubServer {
            db,
        }
    }

    pub(crate) fn indexed_value_ops_sub_server<'b>(
        &'a self,
        lockable_db: TransactionOrDb<'b>,
    ) -> IndexedValueOpsSubServer<'b> {
        IndexedValueOpsSubServer {
            transaction_or_db: lockable_db.clone(),
        }
    }

    pub(crate) fn tabled_value_ops_sub_server<'b>(
        &'a self,
        lockable_db: TransactionOrDb<'b>,
    ) -> TabledValueOpsSubServer<'b> {
        TabledValueOpsSubServer {
            transaction_or_db: lockable_db.clone(),
        }
    }

    pub(crate) fn text_index_ops_sub_server<'b>(
        &'a self,
        lockable_db: TransactionOrDb<'b>,
    ) -> TextIndexOpsSubServer<'b> {
        TextIndexOpsSubServer {
            transaction_or_db: lockable_db.clone(),
        }
    }
}
