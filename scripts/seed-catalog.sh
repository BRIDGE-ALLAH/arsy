#!/usr/bin/env bash
set -Eeuo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
ENV_FILE="$ROOT_DIR/.env"
SEED_FILE="$ROOT_DIR/backend/apps/arsy-control-plane/data/catalog_seed.sql"

for cmd in psql awk; do
  if ! command -v "$cmd" >/dev/null 2>&1; then
    printf 'Missing required command: %s\n' "$cmd" >&2
    exit 1
  fi
done
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
for name in POSTGRES_DB POSTGRES_USER POSTGRES_PASSWORD; do
  value="${!name:-}"
  if [[ -z "$value" || "$value" == "replace-for-local-only" ]]; then
    printf 'Missing or placeholder %s in .env. Run ./scripts/dev-up.sh first.\n' "$name" >&2
    exit 1
  fi
done
if [[ ! "$POSTGRES_DB" =~ ^[A-Za-z0-9_-]+$ || ! "$POSTGRES_USER" =~ ^[A-Za-z0-9_-]+$ ]]; then
  echo "Local database name/user must use letters, digits, underscore, or hyphen." >&2
  exit 1
fi

export PGHOST=127.0.0.1 PGPORT=5432 PGDATABASE="$POSTGRES_DB" PGUSER="$POSTGRES_USER" PGPASSWORD="$POSTGRES_PASSWORD"
echo "Applying repeatable, non-selectable example catalogue rows..."
psql --no-password -v ON_ERROR_STOP=1 -f "$SEED_FILE"
echo "Catalogue seed completed."
