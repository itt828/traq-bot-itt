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

FROM ubuntu:latest
RUN apt-get -y update \
    && apt-get -y install build-essential libssl-dev openssl ca-certificates tesseract-ocr tesseract-ocr-jpn
COPY --from=builder /app/target/release/itt-bot /
COPY --from=builder /app/assets /assets
