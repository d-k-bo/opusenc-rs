FROM rust:slim-bookworm

RUN apt-get update && apt-get install -y libclang-dev libopusenc-dev
RUN rustup component add clippy rustfmt

ENTRYPOINT ["sh", "-c"]
