FROM rust:1.90 AS build
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY . .
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:3.24.1 AS runtime
RUN adduser -D rustapp
COPY --from=build --chown=rustapp /app/target/x86_64-unknown-linux-musl/release/backend /usr/local/bin/app
USER rustapp
ENTRYPOINT ["/usr/local/bin/app"]