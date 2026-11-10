#!/usr/bin/env bash
set -Eeuo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
APP_DIR="$ROOT_DIR/backend/apps/arsy-control-plane"
if [[ -f "$APP_DIR/Cargo.toml" ]]; then
  echo "Loco Control Plane already exists at $APP_DIR"
  exit 0
fi
if ! command -v cargo >/dev/null 2>&1; then
  echo "Rust/Cargo is required. Install rustup using the official Rust instructions." >&2
  exit 1
fi
cd "$ROOT_DIR"
cargo install loco --version 1.3.1 --locked
cargo install sea-orm-cli --version 2.0.0 --locked
mkdir -p "$(dirname "$APP_DIR")"
loco new --path "$APP_DIR" --name arsy_control_plane --db postgres --bg queue-postgres --assets none --allow-in-git-repo
echo "Generated Loco app. Configure config/development.yaml for the local database URI,"
echo "review generated auth/database/worker settings, then run cargo check."
