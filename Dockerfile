FROM rust:1.68.0 as builder-dependencies

RUN rustup component add llvm-tools
ENV RUSTUPBIN=/usr/local/rustup/toolchains/1.68.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin
RUN cargo install cargo-binutils
RUN cargo install rustfilt
ENV RUSTFILT=/usr/local/cargo/bin/rustfilt
RUN apt update && apt-get install -y clang protobuf-compiler

WORKDIR /usr/src
RUN cargo new myapp
COPY Cargo.toml Cargo.lock /usr/src/myapp/
WORKDIR /usr/src/myapp
RUN cargo fetch

FROM builder-dependencies as builder
ENV MYAPPNAME=ondo-server
ENV RUSTFLAGS="-C instrument-coverage"
ENV LLVM_PROFILE_FILE="local-coverage/${MYAPPNAME}.profraw"
COPY . .

FROM builder as checker
RUN cargo check

FROM checker as test
RUN cargo test

FROM test as coverage
RUN $RUSTUPBIN/llvm-profdata merge -sparse local-coverage/${MYAPPNAME}.profraw -o local-coverage/${MYAPPNAME}.profdata
CMD cp -r local-coverage/* coverage/ &&\ 
    $RUSTUPBIN/llvm-cov report -Xdemangler=$RUSTFILT target/debug/${MYAPPNAME}  -instr-profile=coverage/${MYAPPNAME}.profdata

FROM test as dev-builder
RUN cargo build

FROM debian:buster-slim as dev
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=dev-builder /usr/src/myapp/target/debug/${MYAPPNAME} /usr/local/bin/${MYAPPNAME}
CMD ["${MYAPPNAME}"]

FROM test as release-builder
ARG VERSION
ARG COMMIT_NUMBER
ARG TERCEN_DATE

ENV VERSION=$VERSION
ENV COMMIT_NUMBER=$COMMIT_NUMBER
#ENV CURRENT_DATE=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
ENV BUILD_DATE=$BUILD_DATE

ARG BUILD_RUSTFLAGS

ENV RUSTFLAGS=$BUILD_RUSTFLAGS

RUN echo RUSTFLAGS=$RUSTFLAGS

RUN cargo build --release

FROM debian:buster-slim as release
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=release-builder /usr/src/myapp/target/release/${MYAPPNAME} /usr/local/bin/${MYAPPNAME}
CMD ["${MYAPPNAME}"]

# RUST Development Environment
# docker build --target builder -t builder-${MYAPPNAME} .
# docker run -it --rm -v $(pwd):/usr/src/myapp -v$HOME/.cargo/registry/:/usr/local/cargo/registry   builder-${MYAPPNAME}

# CHECK
# docker build --target checker -t checker-${MYAPPNAME} .

# TEST
# docker build --target test -t test-${MYAPPNAME} .

# TEST & ACCESS COVERAGE
# docker build --target coverage -t coverage-${MYAPPNAME} .
# docker run -v $PWD/coverage:/usr/src/myapp/coverage -it --rm coverage-${MYAPPNAME}

# RUN release 
# docker build --target release -t ${MYAPPNAME} .
# docker run -t --rm ${MYAPPNAME} 

# RUN dev   
# docker build --target dev -t dev-${MYAPPNAME} .h
# docker run -t --rm dev-${MYAPPNAME} 

# DETAILED COVERAGE
# $RUSTUPBIN/llvm-cov show -Xdemangler=$RUSTFILT target/debug/${MYAPPNAME} -instr-profile=local-coverage/${MYAPPNAME}.profdata -show-line-counts-or-regions -show-instantiations #-name=add_quoted_string
# https://doc.rust-lang.org/rustc/instrument-coverage.html

# DOCKER DNS PROBLEM:
# $ nmcli dev show | grep 'IP4.DNS'
# In /etc/docker/daemon.json
# {                                                                          
#     "dns": ["x.x.x.x", "y.y.y.y"]                                                                           
# }     
