// index_server_trait_impl.rs
use super::{
    db_error_to_status::{DbErrorOptionToStatus, DbErrorToStatus},
    index_server_trait::IndexServerTrait,
    lockable_db::transaction_maker::LockableTransactionOrDb,
    source_sink::effects_sink::EffectsSink,
};
use crate::db::{
    entity::{index::Index, OndoKey, TableValue},
    reference::{IndexReference, IndexReferenceTrait},
    DbError,
};
use crate::ondo_remote;
use ondo_remote::*;
use tonic::{Request, Response, Status};

impl<'a> Into<IndexReference> for &'a IndexReferenceMessage {
    fn into(self) -> IndexReference {
        IndexReference {
            table_reference: self.table_reference.as_ref().unwrap().into(),
            index_name: self.index_name.clone(),
        }
    }
}
impl Into<IndexReferenceMessage> for IndexReference {
    fn into(self) -> IndexReferenceMessage {
        IndexReferenceMessage {
            table_reference: Some(self.table_reference.into()),
            index_name: self.index_name,
        }
    }
}
impl<'a> Into<Index> for &'a IndexMessage {
    fn into(self) -> Index {
        let reference: IndexReference = self.index_reference.as_ref().unwrap().into();
        let fields: Vec<String> = self.fields.clone();
        Index {
            fields: fields,
            reference,
        }
    }
}
impl Into<IndexMessage> for Index {
    fn into(self) -> IndexMessage {
        let reference: IndexReferenceMessage = self.reference.into();
        let fields: Vec<String> = self.fields.clone();
        IndexMessage {
            fields: fields,
            index_reference: Some(reference),
        }
    }
}

// index_server_trait_impl.rs continued continued
struct IndexedValueReference {
    index_reference: IndexReference,
    key: OndoKey,
}
impl<'a> Into<IndexedValueReference> for &'a IndexedValueReferenceMessage {
    fn into(self) -> IndexedValueReference {
        IndexedValueReference {
            index_reference: self.index_reference.as_ref().unwrap().into(),
            key: self.key.as_ref().unwrap().into(),
        }
    }
}

// index_server_trait_impl.rs continued continued
struct IndexedValueRangeReference {
    index_reference: IndexReference,
    start_key: OndoKey,
    end_key: OndoKey,
}
impl<'a> Into<IndexedValueRangeReference> for &'a IndexedValueRangeReferenceMessage {
    fn into(self) -> IndexedValueRangeReference {
        IndexedValueRangeReference {
            index_reference: self.index_reference.as_ref().unwrap().into(),
            start_key: self.start_key.as_ref().unwrap().into(),
            end_key: self.end_key.as_ref().unwrap().into(),
        }
    }
}

// index_server_trait_impl.rs continued continued
impl<'a> IndexServerTrait for LockableTransactionOrDb<'a> {
    fn create_index(&self, r: Request<IndexMessage>) -> Result<Response<EmptyMessage>, Status> {
        let entity: Index = r.get_ref().into();
        entity
            .reference
            .post_index(&entity, self, self)
            .map_db_err_to_status()?
            .apply_effects(self)
    }

    fn delete_index(
        &self,
        r: Request<IndexReferenceMessage>,
    ) -> Result<Response<EmptyMessage>, Status> {
        let reference: IndexReference = r.get_ref().into();
        reference
            .delete_index(self)
            .map_db_err_to_status()?
            .apply_effects(self)
    }

    fn get_index(
        &self,
        r: Request<IndexReferenceMessage>,
    ) -> Result<Response<IndexMessage>, Status> {
        let reference: IndexReference = r.get_ref().into();
        reference
            .get_index(self)
            .map_db_err_option_to_status()
            .map(|entity| Response::new(entity.into()))
    }

    fn update_index(&self, r: Request<IndexMessage>) -> Result<Response<EmptyMessage>, Status> {
        let entity: Index = r.get_ref().into();
        entity
            .reference
            .put_index(&entity, self, self)
            .map_db_err_to_status()?
            .apply_effects(self)
    }

    fn find_values(
        &self,
        r: Request<IndexedValueReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status> {
        let guard = self.read();
        let indexed_value_reference: IndexedValueReference = r.get_ref().into();
        let reference = indexed_value_reference.index_reference;
        let key_prefix = indexed_value_reference.key;

        let db = guard.inner();
        
        let iterator = reference
            .all_values_with_key_prefix(key_prefix, self, &db)
            .map_db_err_to_status()?;
        let values_result: Result<Vec<TableValue>, DbError> = iterator.collect();
        let values = values_result.map_db_err_to_status()?;
        let json = serde_json::to_string(&values).map_err(|e| Status::internal(e.to_string()))?;
        let response = Response::new(JsonMessage { json });
        Ok(response)
    }

    fn find_values_by_range(
        &self,
        r: Request<IndexedValueRangeReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status> {
        let guard =self.read();
        let indexed_value_range_reference: IndexedValueRangeReference = r.get_ref().into();
        let reference = indexed_value_range_reference.index_reference;
        let start_key_prefix = indexed_value_range_reference.start_key;
        let end_key_prefix = indexed_value_range_reference.end_key;

        let db = guard.inner();

        let iterator = reference
            .all_values_with_key_range(start_key_prefix, end_key_prefix, self, &db)
            .map_db_err_to_status()?;
        let values_result: Result<Vec<TableValue>, DbError> = iterator.collect();
        let values = values_result.map_db_err_to_status()?;
        let json = serde_json::to_string(&values).map_err(|e| Status::internal(e.to_string()))?;
        let response = Response::new(JsonMessage { json });
        Ok(response)
    }
}

// index_server_trait_impl.rs continued
#[cfg(test)]
mod tests {
    use crate::db::entity::{table::Table, table_value::TableValue, DatabaseServer, Domain, Index, ondo_key::OndoKey};
    use crate::db::enums::{index_iterator_requests_factory::IndexIteratorRequestsFactoryEnum};
        use crate::db::reference::Effects;
    use crate::db::reference::{
        CreateTableValueReference, CreateTableValueReferenceTrait, DatabaseServerReference,
        DatabaseServerReferenceTrait, DomainReference, DomainReferenceTrait, IndexReference,
        IndexReferenceTrait, TableReference, TableReferenceTrait, TableValueReference, TableValueReferenceTrait
    };
    use crate::db::server::{lockable_db::LockableDb, source_sink::effects_sink::EffectsTasksSink};
    use serde::{Deserialize, Serialize};
    use crate::db::server::source_sink::effects_sink::EffectsSink;
    use crate::db::server::lockable_db::transaction_maker::{LockableTransactionOrDb};

    pub(crate) fn create_database_server_entity() -> DatabaseServer {
        DatabaseServer::default()
    }

    pub(crate) fn create_domain_entity(database_server_reference: &DatabaseServerReference) -> Domain {
        Domain {
            reference: DomainReference::new(database_server_reference.clone(), "test_domain"),
        }
    }

    pub(crate) fn create_table_entity(domain_reference: &DomainReference) -> Table {
        Table {
            reference: TableReference::new(domain_reference.clone(), "test_table"),
        }
    }

    fn create_index_entity(table_reference: &TableReference) -> Index {
        Index {
            reference: IndexReference::new(table_reference.clone(), "test_index"),
            fields: vec!["city".to_owned()],
        }
    }

    pub(crate) fn create_table_value_reference(table_reference: &TableReference) -> CreateTableValueReference {
        CreateTableValueReference {
            table_reference: table_reference.clone(),
            id: None,
        }
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    pub(crate) struct TestRecord {
        name: String,
        age: u8,
        city: String,
    }
    pub(crate) fn create_test_record1() -> TestRecord {
        TestRecord {
            name: "John".to_owned(),
            age: 30,
            city: "New York".to_owned(),
        }
    }
    pub(crate) fn create_test_record2() -> TestRecord {
        TestRecord {
            name: "Mary".to_owned(),
            age: 20,
            city: "Old York".to_owned(),
        }
    }

    pub(crate) struct TestData {
        lockable_db: LockableTransactionOrDb<'static>,
        database_server_reference: DatabaseServerReference,
        domain_reference: DomainReference,
        table_reference: TableReference,
        // index_reference: IndexReference,
        database_server: DatabaseServer,
        domain: Domain,
        table: Table,
        // index: Index,
    }

    pub(crate) fn setup_test_data() -> TestData {
        // let lockable_db = LockableTransactionOrDb::with_db(LOCKABLE_DB.clone());
        let lockable_db = LockableTransactionOrDb::with_db(LockableDb::in_memory());
        
        let ra = lockable_db;

        // let ra = LockableTransactionOrDb::new(LockableDb::in_memory());

        let database_server = create_database_server_entity();
        let database_server_reference = database_server.reference.clone();

        let domain = create_domain_entity(&database_server_reference);
        let domain_reference = domain.reference.clone();

        let table = create_table_entity(&domain_reference);
        let table_reference = table.reference.clone();

        // let index = create_index_entity(&table_reference);
        // let index_reference = index.reference.clone();

        database_server_reference
            .post_database_server(&database_server, &ra)
            .unwrap()
            .apply_effects(&ra)
            .unwrap();
        domain_reference
            .post_domain(&domain, &ra, &ra)
            .unwrap()
            .apply_effects(&ra)
            .unwrap();
        table_reference
            .post_table(&table, &ra, &ra)
            .unwrap()
            .apply_effects(&ra)
            .unwrap();

        TestData {
            lockable_db: ra,
            database_server_reference,
            domain_reference,
            table_reference,
            // index_reference,
            database_server,
            domain,
            table,
            // index,
        }
    }

    fn create_and_apply_index(test_data: &TestData) -> (Index, Effects) {
        let ra = &test_data.lockable_db;
    
        let index = create_index_entity(&test_data.table_reference);
        let index_reference = &index.reference;
    
        let index_effects = index_reference
            .post_index(&index, ra, ra)
            .unwrap();
        index_effects.apply_effects(ra).unwrap();
    
        (index, index_effects)
    }
    
    fn create_and_apply_record(test_data: &TestData) -> (OndoKey, TableValue, Effects) {
        let ra = &test_data.lockable_db;
    
        let create_table_value_reference = create_table_value_reference(&test_data.table_reference);
        let record1 = create_test_record1();
        let mut value1 = serde_json::to_value(record1).unwrap();
        let (value1_key, value1_effects, value_tasks) = create_table_value_reference
            .post_table_value(&mut value1, ra, ra, ra)
            .unwrap();
        (value1_effects.clone(), value_tasks).apply_effects_queue_tasks(ra).unwrap();
    
        (value1_key, value1, value1_effects)
    }
    
    #[test]
    fn test_get() {
        let test_data = setup_test_data();

        let (value1_key, _value1, _value1_effects) = create_and_apply_record(&test_data);

        let table_reference = test_data.table_reference;
        let ra = &test_data.lockable_db;
        let table_value_reference = TableValueReference {
            table_reference: table_reference.clone(),
            id: value1_key,
        };
        let value1_retrieved = table_value_reference.get_table_value(ra).unwrap().unwrap();

        assert_eq!(
            value1_retrieved,
            serde_json::json!({
                "_id": {"values": [1]},
                "name": "John",
                "age": 30,
                "city": "New York"
            })
        );
    }

    #[test]
    fn test_index_then_populate() {
        let test_data = setup_test_data();
    
        let (_index, _index_effects) = create_and_apply_index(&test_data);
        let (_value1_key, _value1, value1_effects) = create_and_apply_record(&test_data);
    
        let expected_value1_effects_str =
            "[ColumnValueEffect(Put('/domains/test_domain/counters', \
                OndoKey { values: [String('test_table')] }, Number(1))), \
            TableValueEffect(Put('test_domain::/test_table', \
            OndoKey { values: [Number(1)] }, \
            Object {'_id': Object {'values': Array [Number(1)]}, \
                    'age': Number(30), \
                    'city': String('New York'), \
                    'name': String('John')})), \
            IndexValueEffect(Put('test_domain::/test_table/indexes/test_index', \
                    OndoKey { values: [String('New York'), Number(1)] }, \
                    OndoKey { values: [Number(1)] }))]"
                .to_owned()
                .replace('\'', "\"");
        let value1_effects_str = format!("{:?}", value1_effects);
        assert_eq!(value1_effects_str, expected_value1_effects_str);
    }
    
    #[test]
    fn test_index_populated_table() {
        let test_data = setup_test_data();
    
        let (_value1_key, _value1, _value1_effects) = create_and_apply_record(&test_data);
        let (_index, index_effects) = create_and_apply_index(&test_data);
    
        let index_effects_str = format!("{:?}", index_effects);
        let expected_index_effects_str = 
        "[CreateCf('test_domain::/test_table/indexes/test_index'), \
          TableStoredEffect(Put('/domains/test_domain/tables', 'test_table', \
          TableStored { table: Table { reference: TableReference { \
                        domain_reference: DomainReference { domain_name: 'test_domain' }, \
                        table_name: 'test_table' } }, \
                        indexes: {'test_index': Index { reference: IndexReference { \
                                  table_reference: TableReference { \
                                  domain_reference: DomainReference { domain_name: 'test_domain' }, \
                                  table_name: 'test_table' }, \
                                  index_name: 'test_index' }, \
                                  fields: ['city'] }}, \
                        text_indexes: {} })), \
          IndexValueEffect(Put('test_domain::/test_table/indexes/test_index', \
            OndoKey { values: [String('New York'), Number(1)] }, \
            OndoKey { values: [Number(1)] }))]"
        .to_owned()
        .replace('\'', "\"");
        assert_eq!(index_effects_str, expected_index_effects_str);
    }
                              
    #[test]
    fn test_all_values_with_key_prefix_vec() {
        let test_data = setup_test_data();
    
        let (_value1_key, _value1, _value1_effects) = create_and_apply_record(&test_data);
        let (index, _index_effects) = create_and_apply_index(&test_data);
    
        let index_reference = index.reference;
        let ra = &test_data.lockable_db;
        let index_iterator_factory = IndexIteratorRequestsFactoryEnum::new_lockable_db(ra);
    
        let key_prefix: OndoKey = "New York".into();
        let retrieved_all_values = index_reference
            .all_values_with_key_prefix_vec(key_prefix, ra, &index_iterator_factory)
            .unwrap();
    
        assert_eq!(
            retrieved_all_values,
            vec![Ok(serde_json::json!({
                "_id": {"values": [1]},
                "name": "John",
                "age": 30,
                "city": "New York"
            }))]
        );
    
        let key_prefix_fail: OndoKey = "Llanfairpwll".into();
        let retrieved_all_values_fail = index_reference
            .all_values_with_key_prefix_vec(key_prefix_fail, ra, &index_iterator_factory)
            .unwrap();
    
        assert_eq!(retrieved_all_values_fail, vec![]);
    }
                              
}
