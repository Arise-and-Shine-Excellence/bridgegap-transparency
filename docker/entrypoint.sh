#!/bin/sh
# Without LITESTREAM_REPLICA_URL the app runs on its own (no off-site backup).
# With it: restore the database from the replica if this host has none yet,
# then run the app under Litestream so every change is streamed off-site.
set -e
DB=/data/arise_shine.db

if [ -z "$LITESTREAM_REPLICA_URL" ]; then
  echo "LITESTREAM_REPLICA_URL is not set: running WITHOUT off-site backups" >&2
  exec /app/arise_shine
fi

litestream restore -if-db-not-exists -if-replica-exists -config /etc/litestream.yml "$DB"
exec litestream replicate -config /etc/litestream.yml -exec /app/arise_shine
