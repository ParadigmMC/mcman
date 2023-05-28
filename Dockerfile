FROM rust:alpine as builder
WORKDIR /app
RUN rustup target add x86_64-unknown-linux-musl
RUN apk add --no-cache musl-dev

COPY . .
RUN --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --target x86_64-unknown-linux-musl --release && \
    cp /app/target/x86_64-unknown-linux-musl/release/mcman /app/mcman

FROM alpine
RUN apk add --no-cache ca-certificates
COPY --from=builder /app/mcman /usr/bin/mcman