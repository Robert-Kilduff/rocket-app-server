# Use the official Rust image as the base image
FROM rust:latest as builder

# Set the working directory
WORKDIR /rocket-app

# Copy the source code and SQLite database into the container
COPY . .



# Build the application
RUN cargo build --release

#make a smaller image using only needed
FROM alpine:latest

#install runtimes
RUN apk add --no-cache libgcc libstdc++

#set workdir
WORKDIR /rocket-app

# Copy the built application from the previous stage
COPY --from=builder /rocket-app/target/release/rocket-app .
COPY --from=builder /rocket-app/Cargo.toml .
COPY --from=builder /rocket-app/Cargo.lock .

# Set the environment variables
ENV SECRET_KEY=${SECRET_KEY}
ENV JWT_SECRET_KEY=${JWT_SECRET_KEY}
# Expose the application port (e.g., 8000)
EXPOSE 8000

# Set the command to run the application
CMD ["./rocket-app"]

