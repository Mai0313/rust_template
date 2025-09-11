# syntax=docker/dockerfile:1.4

ARG RUST_VERSION=1
FROM rust:${RUST_VERSION}-slim AS builder

LABEL maintainer="Wei Lee <mai@mai0313.com>" \
    org.label-schema.name="rust_template" \
    org.label-schema.vendor="Wei Lee" \
    org.label-schema.schema-version="1.0" \
    com.centurylinklabs.watchtower.stop-signal="SIGINT"

WORKDIR /app

# Install MUSL toolchain and target for static builds
RUN apt-get update && apt-get install -y --no-install-recommends musl-tools pkg-config && \
    rm -rf /var/lib/apt/lists/* && \
    rustup target add x86_64-unknown-linux-musl

# Cache dependencies (with BuildKit caches)
COPY Cargo.toml Cargo.lock ./
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/app/target \
    mkdir -p src && echo 'fn main() { println!("build cache"); }' > src/main.rs && \
    cargo build --release --locked --target x86_64-unknown-linux-musl || true

# Build actual binary (static MUSL)
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo build --release --locked --target x86_64-unknown-linux-musl

########################################################################################

FROM scratch AS prod
LABEL org.opencontainers.image.title="rust_template" \
      org.opencontainers.image.description="rust_template static binary (musl)" \
      org.opencontainers.image.source="https://github.com/<owner>/<repo>"
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rust_template /rust_template
ENTRYPOINT ["/rust_template"]
