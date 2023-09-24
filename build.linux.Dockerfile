# Use a Rust base image
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /app

# Copy the contents of the rustduck directory into the container
COPY ./rustduck /app/rustduck

# Build the rustduck binary
RUN cd /app/rustduck && cargo build --release

# Copy the rustduck_init directory into the container
COPY ./rustduck_init /app/rustduck_init

# Build the rustduck_init binary
RUN cd /app/rustduck_init && cargo build --release

# Copy the rustduck_init binary out of the container to the /app directory
RUN cp /app/rustduck_init/target/release/rustduck_init /app/rustduck_init

# Switch to a scratch (minimal) image to create the final artifact
FROM scratch

# Copy the rustduck_init binary from the builder stage
COPY --from=builder /app/rustduck_init /rustduck_init
# docker build -t builder -f arm.Dockerfile .
# docker create --name builder builder .
# docker cp $id:/rustduck_init .