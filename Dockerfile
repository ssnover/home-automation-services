FROM lukemathwalker/cargo-chef:latest-rust-1.67-slim-bullseye as chef
WORKDIR /app
RUN rustup default beta
RUN apt update && apt install -y \ 
  clang \
  libssl-dev \
  lld \
  pkg-config 

FROM chef as planner
COPY . .
RUN cargo +beta chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo +beta chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin nanoleaf-aurora-driver

FROM debian:bullseye-slim as nanoleaf_driver_runtime
WORKDIR /app

RUN apt-get update -y \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/nanoleaf-aurora-driver nanoleaf-aurora-driver
COPY config.yaml config.yaml
ENTRYPOINT ["./nanoleaf-aurora-driver"]
