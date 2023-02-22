FROM rust:1.67.1-slim AS build
# Download the target for static linking.
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install musl musl-dev musl-tools

# Create a dummy project and build the app's dependencies.
# If the Cargo.toml or Cargo.lock files have not changed,
# we can use the docker build cache and skip these (typically slow) steps.
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Copy the statically-linked binary into a scratch container.
FROM scratch
COPY --from=build /usr/local/cargo/bin/zero2pr .
USER 1000
CMD ["./zero2pr"]
