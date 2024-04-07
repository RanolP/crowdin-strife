FROM --platform=${TARGETARCH} rust:1.77.0 AS chef

WORKDIR /app
RUN CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse cargo install cargo-chef --locked
RUN CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse cargo install cargo-make --locked

FROM chef AS planner

COPY crates crates
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /app/recipe.json .
RUN CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse cargo make prisma generate
RUN CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse cargo build --release --package app

FROM --platform=${TARGETARCH} debian:12.5-slim

COPY --from=builder \
    /app/target/release/app \
    /usr/bin/app

RUN apt-get update
RUN apt-get install -y ca-certificates

ENTRYPOINT [ "/usr/bin/app" ]
