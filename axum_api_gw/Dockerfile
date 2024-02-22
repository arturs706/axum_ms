FROM rust as builder

COPY . /app

WORKDIR /app

RUN cargo build --profile release-lto

FROM gcr.io/distroless/cc-debian12

COPY --from=builder /app/target/release-lto/axum_api_gw /app/axum_api_gw
WORKDIR /app

CMD ["./axum_api_gw"]