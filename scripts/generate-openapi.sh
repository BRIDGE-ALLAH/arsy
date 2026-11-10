#!/usr/bin/env bash
set -Eeuo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
SOURCE="$ROOT_DIR/docs/api/source/openapi.yaml"
OUTPUT="$ROOT_DIR/docs/api/openapi.yaml"
MODE="${1:---check}"

if [[ ! -f "$SOURCE" ]]; then
  echo "Missing OpenAPI source: $SOURCE" >&2
  exit 1
fi

case "$MODE" in
  --write)
    install -D -m 0644 "$SOURCE" "$OUTPUT"
    ;;
  --check)
    if [[ ! -f "$OUTPUT" ]]; then
      echo "Missing generated OpenAPI artifact: $OUTPUT" >&2
      exit 1
    fi
    if ! cmp -s "$SOURCE" "$OUTPUT"; then
      echo "OpenAPI artifact is stale. Run: bash scripts/generate-openapi.sh --write" >&2
      diff -u "$OUTPUT" "$SOURCE" || true
      exit 1
    fi
    ;;
  *)
    echo "Usage: $0 [--write|--check]" >&2
    exit 2
    ;;
esac
