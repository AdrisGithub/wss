FROM rust:latest as builder

COPY . .
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM alpine:latest
RUN apk add gcompat

COPY --from=builder /usr/local/cargo/bin/wss /usr/local/bin/wss
CMD ["/usr/local/bin/wss"]
EXPOSE 8080