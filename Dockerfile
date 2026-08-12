FROM node:24-bookworm-slim AS web-builder

WORKDIR /build/apps/web

COPY apps/web/package.json apps/web/package-lock.json ./

RUN npm ci

COPY apps/web/ ./

RUN npm run build


FROM rust:1.97-bookworm AS rust-builder

WORKDIR /build

COPY Cargo.toml Cargo.lock ./
COPY apps/server/Cargo.toml apps/server/Cargo.toml
COPY crates/database/Cargo.toml crates/database/Cargo.toml

COPY apps/server/src apps/server/src
COPY crates/database/src crates/database/src

RUN cargo build \
    --locked \
    --release \
    -p crumbvote-server


FROM debian:bookworm-slim AS runtime

LABEL org.opencontainers.image.source="https://github.com/Nemeth-Tamas/crumbvote"

RUN apt-get update \
    && apt-get install \
    --yes \
    --no-install-recommends \
    ca-certificates \
    libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/* \
    && mkdir -p \
    /app/web \
    /data/uploads/entries \
    && chown -R 10001:10001 /data

COPY \
    --from=rust-builder \
    /build/target/release/crumbvote-server \
    /app/crumbvote-server

COPY \
    --from=web-builder \
    /build/apps/web/dist \
    /app/web

WORKDIR /

ENV CRUMBVOTE_LISTEN_ADDRESS=0.0.0.0:3000
ENV CRUMBVOTE_WEB_DIRECTORY=/app/web

EXPOSE 3000

VOLUME ["/data"]

USER 10001:10001

ENTRYPOINT ["/app/crumbvote-server"]