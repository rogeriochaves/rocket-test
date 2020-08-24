FROM rust:1.45

RUN rustup default nightly && rustup update

WORKDIR /app

COPY zdockercache/dummy-20200823.rs src/main.rs
COPY zdockercache/Cargo-20200823.toml Cargo.toml
COPY zdockercache/Cargo-20200823.lock Cargo.lock
RUN cargo build --release

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-watch
RUN cargo build --tests

COPY src/dummy.rs src/main.rs
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
RUN cargo build --tests

COPY src src
RUN rm target/release/tpdp-core && cargo build --release && cargo install --path .

COPY . .

CMD ./start.sh
