FROM rust:1.90 AS build
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY . .
RUN cargo build --release

FROM alpine:3.24.1 AS runtime
RUN adduser -D rustapp
COPY --from=build --chown=rustapp /app/target/release/backend /usr/local/bin/app
USER rustapp
ENTRYPOINT ["/usr/local/bin/app"]