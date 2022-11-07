FROM rust:latest
ADD . ./heavy_blockchain
RUN apt update && apt install -y cmake libclang-dev && rm -rf /var/lib/apt/lists/*
RUN rustup toolchain install nightly && rustup default nightly && rustup target add wasm32-unknown-unknown
RUN cd heavy_blockchain && cargo build --release
