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
  JWT_SECRET="$(openssl rand -hex 64)"
  cat >"$ENV_FILE" <<EOF
POSTGRES_DB=arsy
POSTGRES_USER=arsy
POSTGRES_PASSWORD=$DB_PASSWORD
JWT_SECRET=$JWT_SECRET
EOF
  chmod 600 "$ENV_FILE"
  echo "Created local .env with a random database password."
else
  echo "Using existing .env (not modifying secrets)."
fi

# Older local .env files may predate the JWT secret. Add one without overwriting
# any existing database credentials or operator-provided values.
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

# Replace sample placeholders with fresh local-only values. Never silently use
# the public template values as database or JWT credentials.
if ! grep -q '^POSTGRES_PASSWORD=.' "$ENV_FILE" || grep -q '^POSTGRES_PASSWORD=replace-for-local-only

echo "Starting ARSY development dependencies..."
docker compose --env-file "$ENV_FILE" -f "$COMPOSE_FILE" up -d

echo
echo "PostgreSQL container started."
echo "Local connection: 127.0.0.1:5432"
echo "Database: arsy (unless overridden in .env)"
echo "JWT secret: generated locally and stored in ignored .env"
echo "Stop with: ./scripts/dev-down.sh"
 "$ENV_FILE"; then
  DB_PASSWORD="$(openssl rand -hex 32)"
  replace_env_value POSTGRES_PASSWORD "$DB_PASSWORD"
  echo "Generated a random local PostgreSQL password."
fi

if ! grep -q '^JWT_SECRET=.' "$ENV_FILE" || grep -q '^JWT_SECRET=replace-with-random-local-secret

echo "Starting ARSY development dependencies..."
docker compose --env-file "$ENV_FILE" -f "$COMPOSE_FILE" up -d

echo
echo "PostgreSQL container started."
echo "Local connection: 127.0.0.1:5432"
echo "Database: arsy (unless overridden in .env)"
echo "JWT secret: generated locally and stored in ignored .env"
echo "Stop with: ./scripts/dev-down.sh"
 "$ENV_FILE"; then
  JWT_SECRET="$(openssl rand -hex 64)"
  replace_env_value JWT_SECRET "$JWT_SECRET"
  echo "Generated a random local JWT signing secret."
fi

chmod 600 "$ENV_FILE"

echo "Starting ARSY development dependencies..."
docker compose --env-file "$ENV_FILE" -f "$COMPOSE_FILE" up -d

echo
echo "PostgreSQL container started."
echo "Local connection: 127.0.0.1:5432"
echo "Database: arsy (unless overridden in .env)"
echo "JWT secret: generated locally and stored in ignored .env"
echo "Stop with: ./scripts/dev-down.sh"
