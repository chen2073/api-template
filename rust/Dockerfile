FROM rust:1.87

WORKDIR /api

COPY api/ ./

RUN cargo build --release

CMD ["./target/release/api"]
