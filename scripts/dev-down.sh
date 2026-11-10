#!/usr/bin/env bash
set -Eeuo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
ENV_FILE="$ROOT_DIR/.env"
COMPOSE_FILE="$ROOT_DIR/deploy/compose.dev.yaml"

if ! command -v docker >/dev/null 2>&1 || ! docker compose version >/dev/null 2>&1; then
  echo "Docker Engine with the Compose v2 plugin is required." >&2
  exit 1
fi

if [[ ! -f "$ENV_FILE" ]]; then
  echo "No .env found. Nothing to stop."
  exit 0
fi

docker compose --env-file "$ENV_FILE" -f "$COMPOSE_FILE" down
echo "Development dependencies stopped. The database volume was preserved."
