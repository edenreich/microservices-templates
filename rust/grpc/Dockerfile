FROM rust:1.66.0-alpine3.17 AS base
WORKDIR /app
RUN apk add --no-cache --update musl-dev protoc protobuf-dev

FROM base AS protoc-rust-plugin
WORKDIR /app
RUN cargo install protoc-gen-tonic protoc-gen-prost

FROM znly/protoc:0.4.0 AS protoc
WORKDIR /app
COPY --from=protoc-rust-plugin /usr/local/cargo/bin/protoc-gen-tonic /usr/local/cargo/bin/protoc-gen-prost /usr/bin/
COPY protos protos
RUN mkdir -p codegen
RUN protoc -I protos payments.proto \
        --prost_out=codegen \
        --tonic_out=codegen \
        --tonic_opt=no_client
ENTRYPOINT [ "protoc" ]

FROM base AS builder
WORKDIR /app
COPY src src
COPY Cargo.toml Cargo.lock ./
COPY --from=protoc /app/codegen codegen
RUN RUSTFLAGS="$RUSTFLAGS -C target-feature=+crt-static" cargo build --bin payments-server --target x86_64-unknown-linux-musl --release

FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/payments-server /app/payments-server
ENTRYPOINT ["/app/payments-server"]
