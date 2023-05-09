# ondo
RockDB extensions

# dev env

```shell
apt-get install protobuf-compiler
```


# build

```shell
VERSION=0.0.1
COMMIT_NUMBER=0
docker build --target release \
      --build-arg BUILD_RUSTFLAGS="" \
      --build-arg VERSION=$VERSION \
      --build-arg COMMIT_NUMBER=$COMMIT_NUMBER \
      --build-arg BUILD_DATE=$(date -u +"%Y-%m-%dT%H:%M:%SZ") \
      -t tercen/ondo .
```

# todo
- [ ] Feature: Transaction API
    - [ ] Cancelled due to lack of time
      - [ ] - [ ] Parametric types
        - [ ] DbBasic
        - [ ] DbTransaction
        - [ ] DbTransactionFactory
        - [ ] DbMeta
    - [ ] Check: Transansactions are using reentrant mutex
    - [ ] Check: Meta operations exclude all other operations (RWLock)
    - [ ] UMUR departs here...
  - [ ] Fix Transaction lifetime problems
  - [ ] Fix TransactionDB::drop_cf
- [ ] Listing functions need to put into client stream directly instead of collecting a vector
- [ ] Feature: TTL
- [ ] Feature: Revision number
- [ ] Named workers in task queue to keep the order for index operations (they are not queued for now)
- [ ] Text Index tests
- [ ] Name validation for all metadata
- [ ] Bring paging params of Index and TableValue to API. Implementation has them.
- [ ] Make task queue, ondo persistent
- [ ] ?Table Value API: Do not pluck OndoKey from the record. Get it explicitly?
- [ ] Remove unnecessary traits
- [ ] Optimize 7-Bit conversion

