FROM rust:latest

# Install system dependencies

# Set the working directory

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
EXPOSE 8080
# Set the entrypoint
ENTRYPOINT [ "./target/release/dbwebconnect" , "localhost", "$PWD"]