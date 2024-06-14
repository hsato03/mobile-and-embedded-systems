
FROM rust:1.67 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY . .
RUN cargo fetch

RUN cargo build --release

FROM debian:bullseye
WORKDIR /app
COPY --from=builder /app/target/release/temperature /app/temperature
EXPOSE 3000

CMD ["./temperature"]