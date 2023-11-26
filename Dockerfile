FROM rust
WORKDIR /unlock-puzzle-generator
COPY . .
RUN cargo build
RUN cargo test
