FROM mcr.microsoft.com/devcontainers/rust:latest

RUN cargo install trunk \
    && cargo install leptosfmt \
    && cargo install cargo-outdated \
    && rustup target add wasm32-unknown-unknown
