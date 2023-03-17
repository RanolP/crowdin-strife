FROM --platform=${TARGETARCH} rust:1.68.0 AS chef

WORKDIR /app
RUN CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse cargo install cargo-chef --locked
RUN CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse cargo install cargo-make --locked

FROM chef AS planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /app/recipe.json .
RUN CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse cargo make prisma generate
RUN CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse cargo build --release

FROM --platform=${TARGETARCH} debian:11.6-slim

COPY --from=builder \
    /app/target/release/app \
    /usr/bin/app

ENTRYPOINT [ "/usr/bin/app" ]
