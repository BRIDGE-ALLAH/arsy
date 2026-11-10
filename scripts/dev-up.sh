#!/usr/bin/env bash
set -Eeuo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
ENV_FILE="$ROOT_DIR/.env"
COMPOSE_FILE="$ROOT_DIR/deploy/compose.dev.yaml"

for cmd in docker openssl awk grep chmod mktemp mv; do
  if ! command -v "$cmd" >/dev/null 2>&1; then
    printf 'Missing required command: %s\n' "$cmd" >&2
    exit 1
  fi
done

if ! docker compose version >/dev/null 2>&1; then
  echo "Docker Compose v2 is required (docker compose)." >&2
  exit 1
fi

replace_env_value() {
  local key="$1"
  local value="$2"
  local temp_file
  temp_file="$(mktemp "${ENV_FILE}.XXXXXX")"
  awk -F= -v key="$key" -v value="$value" '
    BEGIN { updated = 0 }
    $1 == key { print key "=" value; updated = 1; next }
    { print }
    END { if (!updated) print key "=" value }
  ' "$ENV_FILE" >"$temp_file"
  chmod 600 "$temp_file"
  mv "$temp_file" "$ENV_FILE"
}

if [[ ! -f "$ENV_FILE" ]]; then
  umask 077
  : >"$ENV_FILE"
  replace_env_value POSTGRES_DB arsy
  replace_env_value POSTGRES_USER arsy
  echo "Created local .env."
fi

# Replace absent/sample values; preserve valid values already chosen by the operator.
if ! grep -q '^POSTGRES_PASSWORD=.' "$ENV_FILE" || grep -q '^POSTGRES_PASSWORD=replace-for-local-only$' "$ENV_FILE"; then
  replace_env_value POSTGRES_PASSWORD "$(openssl rand -hex 32)"
  echo "Generated a random local PostgreSQL password."
fi

if ! grep -q '^JWT_SECRET=.' "$ENV_FILE" || grep -q '^JWT_SECRET=replace-with-random-local-secret$' "$ENV_FILE"; then
  JWT_SECRET="$(openssl rand -base64 64 | tr -d '\n')"
  replace_env_value JWT_SECRET "$JWT_SECRET"
  echo "Generated a random local Base64 JWT signing secret."
fi

# This file contains secrets even when supplied by an operator.
chmod 600 "$ENV_FILE"

echo "Starting ARSY development dependencies..."
docker compose --env-file "$ENV_FILE" -f "$COMPOSE_FILE" up -d

echo
echo "PostgreSQL container started."
echo "Local connection: 127.0.0.1:5432"
echo "Database: configured in ignored .env"
echo "JWT secret: generated locally and stored in ignored .env"
echo "Stop with: bash scripts/dev-down.sh"
