FROM rust:1.65.0 as builder
 
WORKDIR /usr/src/myapp
COPY . .

FROM builder as checker
RUN cargo check

FROM checker as dev-builder
RUN cargo build --profile dev

FROM checker as release-builder
RUN cargo build --release

FROM debian:buster-slim as dev
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=dev-builder /usr/src/myapp/target/debug/ondo /usr/local/bin/ondo
CMD ["ondo"]


FROM debian:buster-slim as release
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=release-builder /usr/src/myapp/target/release/ondo /usr/local/bin/ondo
CMD ["ondo"]

# RUN release
# docker build --target release -t ondo .
# docker run -t --rm ondo 

# REN dev   
# docker build --target dev -t dev-ondo .
# docker run -t --rm dev-ondo 
