# Use the official Rust image as the base
FROM rust:latest

# Install dependencies for cross-compilation
RUN apt-get update && apt-get install -y \
    gcc-aarch64-linux-gnu \
    libc6-dev-arm64-cross \
    musl-tools \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add aarch64-unknown-linux-gnu

# Set the default command to run when starting the container
CMD ["bash"]