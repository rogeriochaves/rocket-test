FROM rust:1.45

RUN rustup default nightly && rustup update

WORKDIR /app

COPY zdockercache/dummy-20200823.rs src/main.rs
COPY zdockercache/Cargo-20200823.toml Cargo.toml
COPY zdockercache/Cargo-20200823.lock Cargo.lock
RUN cargo build --release

COPY src/dummy.rs src/main.rs
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src src
RUN rm target/release/tpdp-core && cargo build --release && cargo install

COPY . .

CMD ./start.sh
