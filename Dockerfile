FROM rust:1.67.1 AS build
WORKDIR /usr/src

USER root
RUN cargo new rust-project
WORKDIR /usr/src/rust-project

COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

COPY src ./src
RUN cargo install --path .

# Copy the statically-linked binary into a scratch container.
FROM debian:11-slim
COPY --from=build /usr/local/cargo/bin/zero2pr .
USER 1000
CMD ["./zero2pr"]
