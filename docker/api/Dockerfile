FROM rust:latest as builder
WORKDIR /app
COPY backend_api .

ENV SQLX_OFFLINE=true

RUN cargo fetch
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates
COPY --from=builder /usr/local/cargo/bin/backend_api /usr/local/bin/backend_api
CMD ["backend_api"]
