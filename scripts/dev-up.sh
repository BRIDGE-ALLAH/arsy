#!/usr/bin/env bash
set -Eeuo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
ENV_FILE="$ROOT_DIR/.env"
COMPOSE_FILE="$ROOT_DIR/deploy/compose.dev.yaml"

for cmd in docker openssl awk grep mktemp; do
  if ! command -v "$cmd" >/dev/null 2>&1; then
    printf 'Missing required command: %s\n' "$cmd" >&2
    exit 1
  fi
done

if ! docker compose version >/dev/null 2>&1; then
  echo "Docker Compose v2 is required (docker compose)." >&2
  exit 1
fi

umask 077
if [[ ! -f "$ENV_FILE" ]]; then
  : >"$ENV_FILE"
  echo "Created local .env."
fi
chmod 600 "$ENV_FILE"

read_env_value() {
  local key="$1"
  awk -v key="$key" 'index($0, key "=") == 1 { sub(/^[^=]*=/, ""); print; exit }' "$ENV_FILE"
}

ensure_env_value() {
  local key="$1"
  local generated_value="$2"
  local current_value
  current_value="$(read_env_value "$key")"

  if [[ -n "$current_value" && "$current_value" != "replace-for-local-only" ]]; then
    return
  fi

  local tmp_file
  tmp_file="$(mktemp)"
  awk -v key="$key" -v value="$generated_value" '
    BEGIN { replaced = 0 }
    index($0, key "=") == 1 {
      if (!replaced) {
        print key "=" value
        replaced = 1
      }
      next
    }
    { print }
    END {
      if (!replaced) print key "=" value
    }
  ' "$ENV_FILE" >"$tmp_file"
  chmod 600 "$tmp_file"
  mv "$tmp_file" "$ENV_FILE"
}

ensure_env_value POSTGRES_DB arsy
ensure_env_value POSTGRES_USER arsy
ensure_env_value POSTGRES_PASSWORD "$(openssl rand -hex 32)"
ensure_env_value JWT_SECRET "$(openssl rand -hex 32)"
chmod 600 "$ENV_FILE"

echo "Starting ARSY development dependencies..."
docker compose --env-file "$ENV_FILE" -f "$COMPOSE_FILE" up -d

echo
echo "PostgreSQL is starting on 127.0.0.1:5432."
echo "A local JWT secret is stored in .env with mode 0600."
echo "Start the API with: bash scripts/dev-api.sh"
echo "Stop with: ./scripts/dev-down.sh"
