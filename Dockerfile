FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 

RUN apt-get update && \
    apt-get install -y build-essential

COPY --from=planner /app/recipe.json recipe.json

# build dependencies
RUN cargo chef cook --release --recipe-path recipe.json

# build app
COPY . .
RUN cargo build --release --bin app

FROM debian:bookworm-slim AS runtime

RUN apt-get update && \
    apt-get install software-properties-common python3-launchpadlib -y && \
    # install yt-dlp
    add-apt-repository ppa:tomtomtom/yt-dlp && \
    apt-get update && \
    apt-get install -y yt-dlp

WORKDIR /app
COPY --from=builder /app/target/release/keyboard-cat /keyboard-cat
ENTRYPOINT ["/keyboard-cat"]

LABEL org.opencontainers.image.title "keyboard-cat"
LABEL org.opencontainers.image.description "A very dodgy Discord music bot thrown together out of annoyance."
LABEL org.opencontainers.image.url "https://github.com/kaylendog/keyboard-cat"
LABEL org.opencontainers.image.source "https://github.com/kaylendog/keyboard-cat"
LABEL org.opencontainers.image.version "0.1.0"
LABEL org.opencontainers.image.authors "kaylendog"
