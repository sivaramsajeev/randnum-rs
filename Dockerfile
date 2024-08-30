
FROM rust:slim-bullseye as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release


FROM ubuntu:jammy

WORKDIR /app/

COPY --from=builder /app/target/release/randnum-rs /app/randnum-rs

ENTRYPOINT ["./randnum-rs"]
