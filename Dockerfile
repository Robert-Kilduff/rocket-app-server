# Use the official Rust image as the base image
FROM rust:latest as builder

# Set the working directory
WORKDIR /rocket-app

# Log the working directory
RUN echo "Current work dir is: $(pwd)"

# Copy the source code and SQLite database into the container
COPY . .

# List files to check for Cargo.toml
RUN echo "Files in /rocket-app after copy:" && ls -la /rocket-app && ls -la /rocket-app/src && ls -la /rocket-app/migrations

# Install SQLite development libraries and Diesel CLI
RUN apt-get update && \
    apt-get install -y libsqlite3-dev && \
    cargo install diesel_cli --no-default-features --features sqlite

# Verify installation path of Diesel CLI
RUN echo "Diesel CLI installed at:" && which diesel && ls -la $(dirname $(which diesel))

# Build the application
RUN echo "building application" && cargo build --release

# Minimal base image
FROM ubuntu:latest

# Install necessary runtime dependencies
RUN apt-get update && \
    apt-get install -y libsqlite3-dev libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /rocket-app

# Copy the built application from the previous stage
COPY --from=builder /rocket-app/target/release/rocket-app /usr/local/bin/rocket-app

# Copy the Diesel CLI from the builder stage to the final image
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel

# Copy necessary configuration and migration files
COPY --from=builder /rocket-app/migrations ./migrations
COPY --from=builder /rocket-app/Rocket.toml ./Rocket.toml
COPY --from=builder /rocket-app/.env ./.env
COPY --from=builder /rocket-app/database.sqlite ./database.sqlite

# Log the files in the final stages
RUN echo "files in rocket-app after copying from builder, final stage: " && ls -la /rocket-app

# Log database permissions
RUN echo "Permissions of database.sqlite:" && ls -la /rocket-app/database.sqlite

# Ensure the binaries are executable
RUN chmod +x /usr/local/bin/rocket-app /usr/local/bin/diesel

# Expose the application port (e.g., 8000)
EXPOSE 8000

# Set the environment variables
ENV DATABASE_URL="./database.sqlite"

# Run database migrations and start the application
CMD diesel migration run && /usr/local/bin/rocket-app
