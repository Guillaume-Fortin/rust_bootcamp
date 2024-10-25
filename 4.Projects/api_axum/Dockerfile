# FROM rust:latest

# WORKDIR /usr/src/api_axum
# COPY . .
# RUN cargo install --path .
# CMD ["api_axum"]

ARG RUST_VERSION=1.70.0

FROM --platform=linux/arm64 rust:${RUST_VERSION}-alpine AS builder
#FROM --platform=linux/arm64 rust:${RUST_VERSION}-slim-bullseye AS builder
RUN apk add --no-cache musl-dev

WORKDIR /usr/src/api_axum
COPY . .
RUN cargo install --path .

FROM --platform=linux/arm64 alpine:latest
#FROM gcr.io/distroless/cc-debian12
#FROM --platform=linux/arm64 debian:bullseye-slim AS final
WORKDIR /root/
COPY --from=builder /usr/local/cargo/bin/api_axum .

EXPOSE 8000
ENV RUST_LOG=api_axum=debug,tower_http::trace=debug
# RUN ls -la
ENTRYPOINT ["./api_axum"]