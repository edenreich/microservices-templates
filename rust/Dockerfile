FROM rust:1.66.0-alpine3.17 AS builder
WORKDIR /app
RUN apk add --no-cache --update musl-dev protoc
COPY proto proto
COPY src src
COPY Cargo.toml Cargo.lock ./
COPY build.rs ./
RUN RUSTFLAGS="$RUSTFLAGS -C target-feature=+crt-static" cargo build --bin payments-server --target x86_64-unknown-linux-musl --release

FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/payments-server /app/payments-server
ENTRYPOINT ["/app/payments-server"]
