# Use the official Rust image as a builder
FROM rust:1.72-alpine AS builder

# Install necessary dependencies for building the project
RUN apk add --no-cache musl-dev openssl-dev

# Create a new directory for the project
WORKDIR /usr/src/brigatory

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create a dummy main file to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build the dependencies
RUN cargo build --release
RUN rm -f target/release/deps/brigatory*

# Copy the source code into the container
COPY . .

# Build the project
RUN cargo build --release

# Use a minimal Alpine image for the final stage
FROM alpine:3.16

# Install necessary runtime dependencies
RUN apk add --no-cache libgcc libstdc++ openssl

# Create a non-root user
RUN addgroup -S appgroup && adduser -S appuser -G appgroup

# Copy the compiled binary from the builder
COPY --from=builder /usr/src/brigatory/target/release/brigatory /usr/local/bin/brigatory

# Set the working directory
WORKDIR /app

# Copy the .env file if you use environment variables
COPY .env .env

# Change the ownership of the directory
RUN chown -R appuser:appgroup /app

# Switch to the non-root user
USER appuser

# Ensure the application uses the PORT environment variable
ENV PORT 8080

# Expose the port that the application will run on
EXPOSE 8080

# Set the command to run the application
CMD ["sh", "-c", "brigatory"]
