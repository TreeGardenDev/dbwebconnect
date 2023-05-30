FROM rust:latest

# Install system dependencies

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Build the dependencies
#RUN mkdir src && \
#    echo "fn main() {}" > src/main.rs && \
#    cargo build --release && \
#    rm -r src target

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release
# Set the entrypoint
ENTRYPOINT [ "/app/target/release/dbwebconnect" , "localhost"]