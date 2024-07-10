# Use the official Rust image as the base image
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create a new empty shell project to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build the dependencies
RUN cargo build --release

# Remove the placeholder src directory
RUN rm -rf src

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release

# Use a smaller base image for the final image
FROM rust:slim

# Copy the build artifacts from the build stage
COPY --from=builder /usr/src/app/target/release/orders_problem /usr/local/bin/orders_problem

# Set the startup command to run the binary
CMD ["orders_problem"]