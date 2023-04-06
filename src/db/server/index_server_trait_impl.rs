use super::{
    db_error_to_status::{DbErrorOptionToStatus, DbErrorToStatus},
    index_server_trait::IndexServerTrait,
    rocks_db_accessor::{DbReadLockGuardWrapper, RocksDbAccessor},
    source_sink::EffectsSink,
};
use crate::db::enums::TableStoredIteratorRequestsFactoryEnum;
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

impl IndexServerTrait for RocksDbAccessor {
    fn create_index(&self, r: Request<IndexMessage>) -> Result<Response<EmptyMessage>, Status> {
        let guarded_db = self.guarded_db();
        let factory_enum_db_arc = TableStoredIteratorRequestsFactoryEnum::new_db_arc(guarded_db);
        let entity: Index = r.get_ref().into();
        entity
            .reference
            .post_index(&entity, self, &factory_enum_db_arc)
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
        let guarded_db = self.guarded_db();
        let factory_enum_db_arc = TableStoredIteratorRequestsFactoryEnum::new_db_arc(guarded_db);
        let entity: Index = r.get_ref().into();
        entity
            .reference
            .put_index(&entity, self, &factory_enum_db_arc)
            .map_db_err_to_status()?
            .apply_effects(self)
    }

    fn find_values(
        &self,
        r: Request<IndexedValueReferenceMessage>,
    ) -> Result<Response<JsonMessage>, Status> {
        let guarded_db = self.guarded_db();
        let db_wrapper = DbReadLockGuardWrapper::new(&guarded_db).map_db_err_to_status()?;
        let indexed_value_reference: IndexedValueReference = r.get_ref().into();
        let reference = indexed_value_reference.index_reference;
        let key_prefix = indexed_value_reference.key;
        let iterator = reference
            .all_values_with_key_prefix(key_prefix, self, &db_wrapper)
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
        let guarded_db = self.guarded_db();
        let db_wrapper = DbReadLockGuardWrapper::new(&guarded_db).map_db_err_to_status()?;
        let indexed_value_range_reference: IndexedValueRangeReference = r.get_ref().into();
        let reference = indexed_value_range_reference.index_reference;
        let start_key_prefix = indexed_value_range_reference.start_key;
        let end_key_prefix = indexed_value_range_reference.end_key;
        let iterator = reference
            .all_values_with_key_range(start_key_prefix, end_key_prefix, self, &db_wrapper)
            .map_db_err_to_status()?;
        let values_result: Result<Vec<TableValue>, DbError> = iterator.collect();
        let values = values_result.map_db_err_to_status()?;
        let json = serde_json::to_string(&values).map_err(|e| Status::internal(e.to_string()))?;
        let response = Response::new(JsonMessage { json });
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use crate::db::entity::{table::Table, DatabaseServer, Domain, Index, ondo_key::OndoKey};
    use crate::db::enums::TableStoredIteratorRequestsFactoryEnum;
    use crate::db::reference::TableValueReference;
    use crate::db::reference::TableValueReferenceTrait;
    use crate::db::reference::{
        CreateTableValueReference, CreateTableValueReferenceTrait, DatabaseServerReference,
        DatabaseServerReferenceTrait, DomainReference, DomainReferenceTrait, IndexReference,
        IndexReferenceTrait, TableReference, TableReferenceTrait,
    };
    use crate::db::server::{rocks_db_accessor::RocksDbAccessor, source_sink::EffectsSink};
    use serde::{Deserialize, Serialize};

    fn create_database_server_entity() -> DatabaseServer {
        DatabaseServer::default()
    }

    fn create_domain_entity(database_server_reference: &DatabaseServerReference) -> Domain {
        Domain {
            reference: DomainReference::new(database_server_reference.clone(), "test_domain"),
        }
    }

    fn create_table_entity(domain_reference: &DomainReference) -> Table {
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

    fn create_table_value_reference(table_reference: &TableReference) -> CreateTableValueReference {
        CreateTableValueReference {
            table_reference: table_reference.clone(),
            id: None,
        }
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    struct TestRecord {
        name: String,
        age: u8,
        city: String,
    }
    fn create_test_record1() -> TestRecord {
        TestRecord {
            name: "John".to_owned(),
            age: 30,
            city: "New York".to_owned(),
        }
    }
    fn create_test_record2() -> TestRecord {
        TestRecord {
            name: "Mary".to_owned(),
            age: 20,
            city: "Old York".to_owned(),
        }
    }

    struct TestData {
        rocks_db_accessor: RocksDbAccessor,
        database_server_reference: DatabaseServerReference,
        domain_reference: DomainReference,
        table_reference: TableReference,
        // index_reference: IndexReference,
        database_server: DatabaseServer,
        domain: Domain,
        table: Table,
        // index: Index,
    }

    fn setup() -> TestData {
        let ra = RocksDbAccessor::in_memory();

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
            rocks_db_accessor: ra,
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

    #[test]
    fn test_get() {
        let test_data = setup();
        let ra = &test_data.rocks_db_accessor;

        let table_reference = test_data.table_reference;

        let create_table_value_reference = create_table_value_reference(&table_reference);
        let record1 = create_test_record1();
        let mut value1 = serde_json::to_value(record1.clone()).unwrap();
        let (value1_key, value1_effects) = create_table_value_reference
            .post_table_value(&mut value1, ra, ra, ra)
            .unwrap();
        value1_effects.apply_effects(ra).unwrap();
        println!("TestRecord: {:?}", record1);
        println!("value1_key: {:?}", value1_key);
        let table_value_reference = TableValueReference {
            table_reference: table_reference.clone(),
            id: value1_key,
        };
        let value1_retrieved = table_value_reference.get_table_value(ra).unwrap().unwrap();
        println!("value1_result: {:?}", value1_retrieved);
        assert_eq!(value1, value1_retrieved)
    }

    #[test]
    fn test_index_then_populate() {
        let test_data = setup();
        let ra = &test_data.rocks_db_accessor;
        let guarded_db = ra.guarded_db();

        let table_reference = test_data.table_reference;

        // create an index on unpopulated table
        let index = create_index_entity(&table_reference);
        let index_reference = &index.reference;
        let factory_enum_db_arc = TableStoredIteratorRequestsFactoryEnum::new_db_arc(guarded_db);
        let index_effects = index_reference
            .post_index(&index, ra, &factory_enum_db_arc)
            .unwrap();
        println!("---- Creating index on populated table----");
        println!("index_effects: {:?}", index_effects);
        index_effects.apply_effects(ra).unwrap();

        // create a record
        let create_table_value_reference = create_table_value_reference(&table_reference);
        let record1 = create_test_record1();
        let mut value1 = serde_json::to_value(record1).unwrap();
        let (_value1_key, value1_effects) = create_table_value_reference
            .post_table_value(&mut value1, ra, ra, ra)
            .unwrap();
        // println!("value1_effects: {:?}", value1_effects);
        value1_effects.apply_effects(ra).unwrap();
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
        let value1_effects_str = format! {"{:?}", value1_effects};
        assert_eq!(value1_effects_str, expected_value1_effects_str);
    }

    #[test]
    fn test_index_populated_table() {
        let test_data = setup();
        let ra = &test_data.rocks_db_accessor;
        let guarded_db = ra.guarded_db();

        let table_reference = test_data.table_reference;

        // create a record
        let create_table_value_reference = create_table_value_reference(&table_reference);
        let record1 = create_test_record1();
        let mut value1 = serde_json::to_value(record1).unwrap();
        let (_value1_key, value1_effects) = create_table_value_reference
            .post_table_value(&mut value1, ra, ra, ra)
            .unwrap();
        // println!("value1_effects: {:?}", value1_effects);
        value1_effects.apply_effects(ra).unwrap();
        // create an index on populated table
        let index = create_index_entity(&table_reference);
        let index_reference = &index.reference;
        let factory_enum_db_arc = TableStoredIteratorRequestsFactoryEnum::new_db_arc(guarded_db);
        let index_effects = index_reference
            .post_index(&index, ra, &factory_enum_db_arc)
            .unwrap();
        index_effects.apply_effects(ra).unwrap();
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
                                  fields: ['city'] }} })), \
          IndexValueEffect(Put('test_domain::/test_table/indexes/test_index', \
            OndoKey { values: [String('New York'), Number(1)] }, \
            OndoKey { values: [Number(1)] }))]"
        .to_owned()
        .replace('\'', "\"");
            assert_eq!(index_effects_str, expected_index_effects_str);
    }

    // #[ignore]
    // #[test]
    // fn test_find_by_index() {
    //     let test_data = setup();
    //     let ra = &test_data.rocks_db_accessor;
    //     let guarded_db = ra.guarded_db();

    //     let table_reference = test_data.table_reference;

    //     // create an index on unpopulated table
    //     let index = create_index_entity(&table_reference);
    //     let index_reference = &index.reference;
    //     let factory_enum_db_arc = TableStoredIteratorRequestsFactoryEnum::new_db_arc(guarded_db);
    //     let index_effects = index_reference
    //         .post_index(&index, ra, &factory_enum_db_arc)
    //         .unwrap();
    //     index_effects.apply_effects(ra).unwrap();

    //     // create a record
    //     let create_table_value_reference = create_table_value_reference(&table_reference);
    //     let record1 = create_test_record1();
    //     let mut value1 = serde_json::to_value(record1).unwrap();
    //     let (_value1_key, value1_effects) = create_table_value_reference
    //         .post_table_value(&mut value1, ra, ra, ra)
    //         .unwrap();
    //     value1_effects.apply_effects(ra).unwrap();
    //     let key_prefix: OndoKey = "New York".into();
    //     index_reference.all_values_with_key_prefix(key_prefix, ra, factory_enum_db_arc).unwrap();
    // }

}
