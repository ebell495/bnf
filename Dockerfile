# FROM rust:latest as builder
# RUN rustup toolchain install nightly && cargo install cargo-fuzz
# COPY . /bnf/
# WORKDIR /bnf/fuzz
# RUN cargo +nightly fuzz build

# FROM debian:bullseye-slim
# COPY --from=builder /bnf/fuzz/target/x86_64-unknown-linux-gnu/release/ .

FROM rust:latest as builder
# RUN rustup toolchain install nightly && cargo install cargo-fuzz
COPY . /bnf/
WORKDIR /bnf/fuzzing
RUN cargo build

FROM debian:bullseye-slim
COPY --from=builder /bnf/fuzzing/target/debug .