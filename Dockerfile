FROM rust:1.63.0 as builder
WORKDIR /usr/src/inmemodb
COPY . .
RUN cargo +nightly install -Z no-index-update --path .
 
FROM alpine
COPY --from=builder /usr/local/cargo/bin/inmemodb /usr/local/bin/inmemodb

ENTRYPOINT ["inmemodb"]