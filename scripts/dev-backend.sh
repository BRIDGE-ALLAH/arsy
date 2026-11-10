#!/usr/bin/env bash
set -Eeuo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ENV_FILE="$ROOT_DIR/.env"
APP_DIR="$ROOT_DIR/backend/apps/arsy-control-plane"

if [[ ! -f "$APP_DIR/Cargo.toml" ]]; then
  echo "Loco Control Plane source is missing at $APP_DIR" >&2
  exit 1
fi

# Ensure PostgreSQL exists and the local-only environment file is initialized.
bash "$ROOT_DIR/scripts/dev-up.sh"

read_env() {
  local key="$1"
  awk -F= -v key="$key" '$1 == key { sub(/^[^=]*=/, ""); print; exit }' "$ENV_FILE"
}

DB_NAME="$(read_env POSTGRES_DB)"
DB_USER="$(read_env POSTGRES_USER)"
DB_PASSWORD="$(read_env POSTGRES_PASSWORD)"
JWT_SECRET="$(read_env JWT_SECRET)"

for pair in "POSTGRES_DB:$DB_NAME" "POSTGRES_USER:$DB_USER" "POSTGRES_PASSWORD:$DB_PASSWORD" "JWT_SECRET:$JWT_SECRET"; do
  if [[ "${pair#*:}" == "" ]]; then
    echo "Missing ${pair%%:*} in local .env. Run scripts/dev-up.sh again." >&2
    exit 1
  fi
done

# Refuse unexpected syntax rather than evaluating arbitrary dotenv content or
# constructing an ambiguous connection URI.
for value in "$DB_NAME" "$DB_USER" "$DB_PASSWORD" "$JWT_SECRET"; do
  if [[ ! "$value" =~ ^[A-Za-z0-9_-]+$ ]]; then
    echo "Local .env contains a value with unsupported characters; use URL-safe values." >&2
    exit 1
  fi
done

export DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@127.0.0.1:5432/${DB_NAME}"
export QUEUE_URL="$DATABASE_URL"
export JWT_SECRET
export BINDING="${BINDING:-127.0.0.1}"
export PORT="${PORT:-5150}"

cd "$APP_DIR"
exec cargo run -- start "$@"
