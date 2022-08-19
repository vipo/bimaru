FROM rust:buster as builder
WORKDIR /usr/src/bimaru
COPY . .
# RUN cargo test
# RUN cargo build --release
RUN cargo test
RUN cargo build --release
RUN cargo install --path .
 
FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/bimaru /usr/local/bin/bimaru
CMD ["bimaru"]