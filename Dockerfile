# FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
# WORKDIR /app

# FROM chef AS planner
# COPY . .
# RUN cargo chef prepare --recipe-path recipe.json

# FROM chef AS builder 
# COPY --from=planner /app/recipe.json recipe.json
# # Build dependencies - this is the caching Docker layer!
# RUN cargo chef cook --release --recipe-path recipe.json
# # Build application
# COPY . .
# RUN cargo build --release

# # We do not need the Rust toolchain to run the binary!
# FROM gcr.io/distroless/base AS runtime
# COPY --from=builder /app/target/release/itt-bot /usr/local/bin/itt-bot
# # ENTRYPOINT ["sh -c", "/usr/local/bin/itt-bot"]

FROM rust:1.70.0

WORKDIR /app

COPY . .
RUN cargo build --release