# One image: the server binary, the site assets and Litestream for backups.
#   docker build -t arise-shine .

FROM rust:1.96-trixie AS builder
RUN rustup target add wasm32-unknown-unknown \
 && cargo install cargo-leptos --locked --version 0.3.7
WORKDIR /app
COPY . .
RUN cargo leptos build --release

FROM litestream/litestream:0.3.13 AS litestream

FROM debian:trixie-slim
RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates \
 && rm -rf /var/lib/apt/lists/* \
 && useradd --system --create-home app \
 && mkdir -p /data && chown app /data
COPY --from=litestream /usr/local/bin/litestream /usr/local/bin/litestream
COPY --from=builder /app/target/release/arise_shine /app/arise_shine
COPY --from=builder /app/target/site /app/site
COPY litestream.yml /etc/litestream.yml
COPY docker/entrypoint.sh /app/entrypoint.sh

WORKDIR /app
USER app
ENV LEPTOS_SITE_ROOT=site \
    LEPTOS_SITE_ADDR=0.0.0.0:3000 \
    LEPTOS_ENV=PROD \
    DATABASE_URL=sqlite:///data/arise_shine.db \
    RUST_LOG=info,sqlx=warn
EXPOSE 3000
VOLUME /data
ENTRYPOINT ["/app/entrypoint.sh"]
