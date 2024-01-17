FROM rust:1.74 AS builder
WORKDIR /app

COPY Cargo.toml .
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY src src
RUN cargo build --release

RUN strip target/release/todo-app-api-rust

FROM gcr.io/distroless/cc-debian12 AS release
WORKDIR /app

COPY --from=builder /app/target/release/todo-app-api-rust .

ENV LISTENER.HOST=0.0.0.0
ENV LISTENER.PORT=8080

EXPOSE 8080

CMD ["./todo-app-api-rust"]