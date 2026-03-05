FROM rust:1.74-slim-bookworm

RUN apt-get update && apt-get install -y \
    git \
    rsync \
    && rm -rf /var/lib/apt/lists/*
