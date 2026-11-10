#!/usr/bin/env bash
set -Eeuo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
ENV_FILE="$ROOT_DIR/.env"
COMPOSE_FILE="$ROOT_DIR/deploy/compose.dev.yaml"

for cmd in docker openssl; do
  if ! command -v "$cmd" >/dev/null 2>&1; then
    printf 'Missing required command: %s\n' "$cmd" >&2
    exit 1
  fi
done

if ! docker compose version >/dev/null 2>&1; then
  echo "Docker Compose v2 is required (docker compose)." >&2
  exit 1
fi

if [[ ! -f "$ENV_FILE" ]]; then
  DB_PASSWORD="$(openssl rand -hex 32)"
  umask 077
  cat >"$ENV_FILE" <<EOF
POSTGRES_DB=arsy
POSTGRES_USER=arsy
POSTGRES_PASSWORD=$DB_PASSWORD
EOF
  chmod 600 "$ENV_FILE"
  echo "Created local .env with a random database password."
else
  echo "Using existing .env (not modifying secrets)."
fi

echo "Starting ARSY development dependencies..."
docker compose --env-file "$ENV_FILE" -f "$COMPOSE_FILE" up -d

echo
echo "PostgreSQL container started."
echo "Local connection: 127.0.0.1:5432"
echo "Database: arsy (unless overridden in .env)"
echo "Stop with: ./scripts/dev-down.sh"
