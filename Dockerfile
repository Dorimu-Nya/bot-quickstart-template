# syntax=docker/dockerfile:1

ARG ALPINE_VERSION=3.23

FROM rust:1.97-alpine${ALPINE_VERSION} AS builder

WORKDIR /app

RUN apk add --no-cache build-base jq openssl-dev pkgconf

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN set -eu; \
    package_name="$(cargo metadata --locked --no-deps --format-version 1 | jq -r '.packages[0].name')"; \
    cargo build --locked --release --message-format=json-render-diagnostics > /tmp/cargo-build.json; \
    executable="$(jq -r --arg package_name "$package_name" \
        'select(.reason == "compiler-artifact") \
        | select(.target.name == $package_name) \
        | select(.target.kind | index("bin")) \
        | .executable // empty' \
        /tmp/cargo-build.json | tail -n 1)"; \
    test -n "$executable"; \
    strip "$executable"; \
    mkdir -p /out; \
    cp "$executable" /out/app

FROM alpine:${ALPINE_VERSION} AS runtime

RUN apk add --no-cache ca-certificates libcrypto3 libgcc libssl3

WORKDIR /app

COPY --from=builder /out/app /usr/local/bin/app

# config.toml contains credentials, so mount it at runtime instead of baking it
# into the image: -v "$(pwd)/config.toml:/app/config.toml:ro"
ENV RUST_LOG=info

EXPOSE 3000

USER 65534:65534

ENTRYPOINT ["/usr/local/bin/app"]
