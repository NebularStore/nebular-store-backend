FROM rust:latest
WORKDIR /build
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
EXPOSE 8080
CMD ["cargo", "run", "--release"]