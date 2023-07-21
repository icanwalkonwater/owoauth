FROM rust:1-slim-bookworm

RUN rustup toolchain install -c rust-src,cargo,clippy,rustfmt,rust-analyzer stable nightly
RUN apt -y update &&\
    apt -y install git pre-commit

RUN useradd -m dev
USER dev
