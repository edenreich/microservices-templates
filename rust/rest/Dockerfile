FROM rust:1.66.0-alpine3.17 AS base
WORKDIR /app
RUN apk add --no-cache --update musl-dev

FROM base AS builder
WORKDIR /app
COPY src src
COPY Cargo.toml Cargo.lock ./
RUN RUSTFLAGS="$RUSTFLAGS -C target-feature=+crt-static" cargo build --bin server --target x86_64-unknown-linux-musl --release

FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/server /app/server
ENTRYPOINT ["/app/server"]
