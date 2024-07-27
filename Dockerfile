# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory
WORKDIR /rocket-app

# Copy the source code and SQLite database into the container
COPY . .

# Build the application
RUN cargo build --release

# Set the environment variables
ENV SECRET_KEY=${SECRET_KEY}
ENV JWT_SECRET_KEY=${JWT_SECRET_KEY}
# Expose the application port (e.g., 8000)
EXPOSE 8000

# Set the command to run the application
CMD ["./target/release/rocket-app"]

