FROM rust:latest
RUN cargo install cargo-build-deps
RUN cargo install diesel_cli --no-default-features --features postgres
RUN USER=root cargo new tb
WORKDIR /tb
COPY Cargo.toml Cargo.lock ./
RUN cargo build-deps --release
COPY src ./src
RUN cargo build --release
COPY .env ./
COPY migrations ./migrations
CMD ./target/release/telegram-rm
