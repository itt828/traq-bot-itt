FROM rust:1.70.0 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# We do not need the Rust toolchain to run the binary!
FROM gcr.io/distroless/base AS runtime
COPY --from=builder /app/target/release/itt-bot /usr/local/bin/itt-bot

# ENTRYPOINT ["sh -c", "/usr/local/bin/itt-bot"]

# FROM rust:1.70.0

# WORKDIR /app

# COPY . .
# RUN cargo build --release