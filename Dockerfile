FROM rust:latest
WORKDIR /build
COPY . .
RUN cargo build --release
EXPOSE 8080
CMD ["cargo", "run", "--release"]