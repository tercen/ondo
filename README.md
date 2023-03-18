# ondo
RockDB extentions

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