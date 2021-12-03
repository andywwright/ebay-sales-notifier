FROM rust:latest
RUN cargo install cargo-build-deps
RUN USER=root cargo new ebay-sales-notifier
WORKDIR /ebay-sales-notifier
COPY Cargo.toml Cargo.lock ./
RUN cargo build-deps --release
COPY src ./src
COPY .conf.yaml ./
RUN cargo run  --release
