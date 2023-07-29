# Start from the latest Rust base image
FROM rust:1.70.0 as chef
RUN cargo install cargo-chef
# Create a new empty shell project
WORKDIR /usr/src/app

FROM chef as planner
# Copy our source code
COPY ./ ./
# Generate info for caching dependencies
RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS builder
COPY --from=planner /usr/src/app/recipe.json recipe.json
# Build & cache dependencies
RUN cargo chef cook --release --recipe-path recipe.json

# Install protoc
RUN apt update && apt upgrade -y
RUN apt install -y protobuf-compiler libprotobuf-dev

# Copy source code from previous stage
COPY . .
# Build for release
RUN cargo build  --release

# Start a new stage
FROM rust:1.70.0

# Copy the binary from builder to this new stage
COPY --from=builder /usr/src/app/target/release/api-server /usr/local/bin

# Port
EXPOSE 7070

# Command to run the binary
ENTRYPOINT ["api-server"]