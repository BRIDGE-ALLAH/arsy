#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

if ! command -v loco >/dev/null 2>&1; then
  echo "Loco CLI is missing. Install it with: cargo install loco --version 1.3.1 --locked" >&2
  exit 1
fi

app_dir="backend/apps/arsy-control-plane"
mkdir -p "$(dirname "$app_dir")"
if [[ -e "$app_dir" ]]; then
  echo "Refusing to overwrite existing app directory: $app_dir" >&2
  exit 1
fi

loco new \
  --name arsy-control-plane \
  --path backend/apps \
  --db postgres \
  --bg queue-postgres \
  --assets none \
  --allow-in-git-repo

test -f "$app_dir/Cargo.toml"
test -f "$app_dir/config/development.yaml"
test -d "$app_dir/src"
