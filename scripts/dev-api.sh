#!/usr/bin/env bash
set -Eeuo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
ENV_FILE="$ROOT_DIR/.env"

if [[ ! -f "$ENV_FILE" ]]; then
  echo "Missing .env. Run ./scripts/dev-up.sh first." >&2
  exit 1
fi

read_env_value() {
  local key="$1"
  awk -v key="$key" 'index($0, key "=") == 1 { sub(/^[^=]*=/, ""); print; exit }' "$ENV_FILE"
}

POSTGRES_DB="$(read_env_value POSTGRES_DB)"
POSTGRES_USER="$(read_env_value POSTGRES_USER)"
POSTGRES_PASSWORD="$(read_env_value POSTGRES_PASSWORD)"
JWT_SECRET="$(read_env_value JWT_SECRET)"

for name in POSTGRES_DB POSTGRES_USER POSTGRES_PASSWORD JWT_SECRET; do
  value="${!name:-}"
  if [[ -z "$value" || "$value" == "replace-for-local-only" ]]; then
    printf 'Missing or placeholder value for %s in .env. Run ./scripts/dev-up.sh to generate local values.\n' "$name" >&2
    exit 1
  fi
done

# Restrict local identifiers to URL-safe values; dev-up.sh generates passwords
# as hexadecimal, so no secret needs to be echoed or URL-encoded in this script.
if [[ ! "$POSTGRES_DB" =~ ^[A-Za-z0-9_-]+$ || ! "$POSTGRES_USER" =~ ^[A-Za-z0-9_-]+$ || ! "$POSTGRES_PASSWORD" =~ ^[A-Za-z0-9._~-]+$ ]]; then
  echo "Local PostgreSQL credentials must use URL-safe characters. Regenerate them with ./scripts/dev-up.sh after backing up any disposable local data." >&2
  exit 1
fi

export DATABASE_URL="postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@127.0.0.1:5432/${POSTGRES_DB}"
export QUEUE_URL="$DATABASE_URL"
export JWT_SECRET

cd "$ROOT_DIR/backend/apps/arsy-control-plane"
exec cargo loco start
