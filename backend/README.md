# ARSY Backend

The backend is a Rust workspace. The first application is the Loco.rs Control Plane API, with a PostgreSQL-backed durable worker queue.

## Workspace layout

- `apps/arsy-control-plane/`: Loco.rs API and application runtime.
- `apps/arsy-control-plane/migration/`: SeaORM migrations.
- `Cargo.toml`: workspace root and shared dependency declarations.

## Development

Prerequisites: stable Rust toolchain (see `rust-toolchain.toml`), Cargo, and PostgreSQL (start the local database with `./scripts/dev-up.sh` from the repository root).

From the repository root:

```bash
cd backend
cargo fmt --all
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Run the API from the application directory after configuring its database URL:

```bash
cd backend/apps/arsy-control-plane
DATABASE_URL='postgres://<user>:<password>@localhost:5432/<database>' cargo run --bin arsy_control_plane-cli -- start
```

Never put a real database password in shell history, documentation, or committed files. Use a local environment file with restrictive permissions.

## Architecture rules

- Keep controllers thin; place business use cases outside HTTP handlers.
- Do not put provider SDK calls directly in controllers.
- Persist operations before dispatching external side effects.
- Keep secrets out of logs and source control.
- Add migrations and tests with each domain feature.

## Status

The Loco application scaffold exists. Domain modules, production authentication/authorization, provisioning workflows, and the broader control-plane capabilities remain in phased implementation.
