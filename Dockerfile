# Use the official Rust image as the base image
FROM rust:latest as builder

# Set the working directory
WORKDIR /rocket-app

#logging the work dir
RUN echo "Current work dir is: $(pwd)"

# Copy the source code and SQLite database into the container
COPY Cargo.toml Cargo.lock ./

#continue copying source
COPY src ./src

# Copy other files
COPY rocket.toml ./
COPY diesel.toml ./
COPY database.sqlite ./
COPY migrations ./migrations


#listing files to check for Cargo.toml
RUN echo "Files after copy rocket-app: " && ls -la /rocket-app


# Build the application
RUN echo "building application" && cargo build --release

#make a smaller image using only needed
FROM alpine:latest

#install runtimes
RUN apk add --no-cache libgcc libstdc++

#set workdir
WORKDIR /rocket-app

#logging the work dir after setting to /rocket-app
RUN echo "Current work dir is: $(pwd)"

# Copy the built application from the previous stage
COPY --from=builder /rocket-app/target/release/rocket-app .
COPY --from=builder /rocket-app/Cargo.toml .
COPY --from=builder /rocket-app/Cargo.lock .

#Logging the files in the final stages
RUN echo "files in rocket-app after copying from builder, final stage: " && ls -la /rocket-app
# Set the environment variables
ENV SECRET_KEY=${SECRET_KEY}
ENV JWT_SECRET_KEY=${JWT_SECRET_KEY}
# Expose the application port (e.g., 8000)
EXPOSE 8000

# Set the command to run the application
CMD ["./rocket-app"]

