FROM rust:1.90-slim-trixie AS build
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY . .
RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN cargo build --release

FROM debian:trixie-slim AS runtime
RUN useradd -m -s /bin/bash rustapp && apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
COPY --from=build --chown=rustapp /app/target/release/backend /usr/local/bin/app
USER rustapp
ENTRYPOINT ["/usr/local/bin/app"]