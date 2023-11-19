FROM rust:latest as builder
WORKDIR /usr/src/wss
COPY . .
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo install --target x86_64-unknown-linux-musl --path .


FROM alpine:latest
# RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
RUN apk add gcompat
COPY --from=builder /usr/local/cargo/bin/wss /usr/local/bin/wss


CMD ["/usr/local/bin/wss"]
EXPOSE 8080