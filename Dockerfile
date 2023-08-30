FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release


FROM debian:bullseye-slim
RUN apt-get update && \
    apt-get install tesseract-ocr -y && \
    apt-get install tesseract-ocr-jpn -y

COPY --from=builder /app/target/release/itt-bot /