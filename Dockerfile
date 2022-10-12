FROM rust:1.63.0 as builder
RUN USER=root cargo new --bin inmemodb
WORKDIR /inmemodb
# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

# build for release
RUN rm ./target/release/deps/inmemodb*
RUN cargo build --release



 
FROM debian:buster-slim
WORKDIR /app
COPY --from=builder /inmemodb/target/release/inmemodb .


CMD ["/app/inmemodb"]