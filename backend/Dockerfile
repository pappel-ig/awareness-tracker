FROM rust:1.90 AS build
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
RUN adduser --disabled-password --gecos "" rustapp
COPY --from=build --chown=rustapp /app/target/release/backend /usr/local/bin/app
USER rustapp
ENTRYPOINT ["/usr/local/bin/app"]