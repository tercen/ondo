// text_index_reference/mod.rs
use crate::db::tasks::task::Tasks;
use crate::db::{
    db_error::DbError,
    db_error::DbResult,
    entity::text_index::TextIndex,
    reference::{
        effect::Effects,
        requests::table_stored_requests::TableStoredRequests,
        table_reference::{stored::TableStoredReferenceTrait, TableReference},
    },
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub(crate) struct TextIndexReference {
    pub(crate) table_reference: TableReference,
    pub(crate) index_name: String,
}

impl TextIndexReference {
    pub(crate) fn build(domain_name: &str, table_name: &str, index_name: &str) -> Self {
        Self {
            table_reference: TableReference::build(domain_name, table_name),
            index_name: index_name.to_owned(),
        }
    }

    pub(crate) fn new(table_reference: TableReference, index_name: &str) -> Self {
        TextIndexReference {
            table_reference,
            index_name: index_name.to_owned(),
        }
    }

    pub(crate) fn to_table_reference(&self) -> TableReference {
        self.table_reference.clone()
    }

    pub(crate) fn get_text_index(
        &self,
        parent_requests: &dyn TableStoredRequests,
    ) -> DbResult<Option<TextIndex>> {
        let table_stored_opt = self.table_reference.get_table_stored(parent_requests)?;
        let table_stored = table_stored_opt.ok_or(DbError::TableNotInitialized)?;
        Ok(table_stored.text_indexes.get(&self.index_name).cloned())
    }

    pub(crate) fn put_text_index(
        &self,
        text_index: &TextIndex,
        parent_requests: &dyn TableStoredRequests,
    ) -> DbResult<(Effects, Tasks)> {
        let table_stored_opt = self.table_reference.get_table_stored(parent_requests)?;
        let mut table_stored = table_stored_opt.ok_or(DbError::TableNotInitialized)?;
        let result = table_stored
            .text_indexes
            .insert(self.index_name.clone(), text_index.clone());
        if result == None {
            Err(DbError::IndexNotInitialized)
        } else {
            let mut effects: Effects = Vec::new();
            effects.extend(self.table_reference.put_table_stored(&table_stored)?);
            let tasks = vec![
                text_index.deindex_related_table_value_keys(),
                text_index.index_related_table_values(),
            ];
            Ok((effects, tasks))
        }
    }

    pub(crate) fn post_text_index(
        &self,
        text_index: &TextIndex,
        parent_requests: &dyn TableStoredRequests,
    ) -> DbResult<(Effects, Tasks)> {
        let table_stored_opt = self.table_reference.get_table_stored(parent_requests)?;
        let mut table_stored = table_stored_opt.ok_or(DbError::TableNotInitialized)?;
        let result = table_stored
            .text_indexes
            .insert(self.index_name.clone(), text_index.clone());
        if result == None {
            // new index
            let effects = self.table_reference.put_table_stored(&table_stored)?;
            let tasks = vec![text_index.index_related_table_values()];
            Ok((effects, tasks))
        } else {
            Err(DbError::AlreadyExists)
        }
    }

    pub(crate) fn delete_text_index(
        &self,
        parent_requests: &dyn TableStoredRequests,
    ) -> DbResult<(Effects, Tasks)> {
        let table_stored_opt = self.table_reference.get_table_stored(parent_requests)?;
        let mut table_stored = table_stored_opt.ok_or(DbError::TableNotInitialized)?;
        let index_opt = table_stored.text_indexes.remove(&self.index_name);
        match index_opt {
            Some(index) => {
                let effects: Effects = self.table_reference.put_table_stored(&table_stored)?;
                let tasks = vec![index.deindex_related_table_value_keys()];
                Ok((effects, tasks))
            }
            None => Ok((Default::default(), Default::default())),
        }
    }
}
