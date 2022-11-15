FROM rust:1.65.0 as builder
 
WORKDIR /usr/src/myapp
RUN rustup component add llvm-tools-preview
ENV RUSTUPBIN=/usr/local/rustup/toolchains/1.65.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin
RUN cargo install rustfilt
ENV RUSTFILT=/usr/local/cargo/bin/rustfilt
COPY . .

FROM builder as checker
RUN cargo check

FROM checker as test
RUN RUSTFLAGS="-C instrument-coverage" LLVM_PROFILE_FILE="local-coverage/ondo.profraw" cargo test

FROM test as coverage
RUN $RUSTUPBIN/llvm-profdata merge -sparse local-coverage/ondo.profraw -o local-coverage/ondo.profdata
CMD cp -r local-coverage/* coverage/ &&\ 
    $RUSTUPBIN/llvm-cov report -Xdemangler=$RUSTFILT target/debug/ondo  -instr-profile=coverage/ondo.profdata

FROM test as dev-builder
RUN cargo build

FROM debian:buster-slim as dev
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=dev-builder /usr/src/myapp/target/debug/ondo /usr/local/bin/ondo
CMD ["ondo"]

FROM test as release-builder
RUN cargo build --release

FROM debian:buster-slim as release
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=release-builder /usr/src/myapp/target/release/ondo /usr/local/bin/ondo
CMD ["ondo"]

# CHECK
# docker build --target checker -t check-ondo .

# TEST
# docker build --target test -t test-ondo .

# TEST & ACCESS COVERAGE
# docker build --target coverage -t coverage-ondo .
# docker run -v $PWD/coverage:/usr/src/myapp/coverage -it --rm coverage-ondo

# RUN release 
# docker build --target release -t ondo .
# docker run -t --rm ondo 

# RUN dev   
# docker build --target dev -t dev-ondo .
# docker run -t --rm dev-ondo 

# DETAILED COVERAGE
#$RUSTUPBIN/llvm-cov show -Xdemangler=$RUSTFILT target/debug/ondo -instr-profile=local-coverage/ondo.profdata -show-line-counts-or-regions -show-instantiations #-name=add_quoted_string
