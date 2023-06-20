FROM rust:1.70.0 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc AS runtime
COPY --from=builder /app/target/release/itt-bot /