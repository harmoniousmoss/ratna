# Use the official Rust image for the building stage
FROM rust:1.72 AS builder
WORKDIR /usr/src/rust_keeper

# Copy the source code and .env file into the container
COPY . .

# Build the release version of the application
RUN cargo build --release

# Use a newer Ubuntu image to run the application
FROM ubuntu:24.10

# Install necessary packages for runtime
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /usr/src/rust_keeper/target/release/rust_keeper /usr/local/bin/rust_keeper

# Copy the .env file to the container
COPY --from=builder /usr/src/rust_keeper/.env /.env

# Set environment variables
ENV RUST_LOG=info

# Expose the port your app runs on
EXPOSE 8080

# Command to run the application
CMD ["rust_keeper"]
