FROM rust:1.40-alpine

RUN mkdir -p /ratel
WORKDIR /ratel
COPY  . ../src
COPY . ../Cargo.toml


RUN cargo build --release

ENTRYPOINT ['/ratel/scripts/pair_rush.sh']