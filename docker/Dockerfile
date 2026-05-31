FROM rust:1.96.0-bookworm AS builder

LABEL maintainer="Wei Lee <mai@mai0313.com>" \
    org.label-schema.name="rust_template" \
    org.label-schema.vendor="Wei Lee" \
    org.label-schema.schema-version="1.0" \
    com.centurylinklabs.watchtower.stop-signal="SIGINT"

WORKDIR /app
COPY . .

# Cache mounts:
#   /usr/local/cargo/registry, /usr/local/cargo/git -> avoid re-downloading crates
#   /app/target                                     -> preserve incremental build artifacts
# /app/target lives in the cache mount, so we cp the binary out to a real layer path
# before the cache detaches; the prod stage copies from that path.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/app/target \
    cargo build --release --locked \
    && cp target/release/rust_template /usr/local/bin/rust_template

########################################################################################

FROM ubuntu:26.04 AS prod
COPY --from=builder /usr/local/bin/rust_template /usr/local/bin/rust_template
ENTRYPOINT ["rust_template"]
