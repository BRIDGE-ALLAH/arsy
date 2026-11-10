# ARSY Backend

The backend is a Rust codebase. The first application is the Loco.rs Control Plane API.

## Scaffold the Control Plane

Prerequisites: stable Rust toolchain, Cargo, PostgreSQL (provided locally through scripts/dev-up.sh), and the Loco/SeaORM CLIs.

From the repository root:

~~~bash
cargo install loco
cargo install sea-orm-cli
loco new \
  --path backend/apps/arsy-control-plane \
  --name arsy_control_plane \
  --db postgres \
  --bg queue-postgres \
  --assets none \
  --allow-in-git-repo
~~~

The command uses Loco's official generator for a database-backed JSON API and PostgreSQL-backed durable worker queue. Review the generated configuration before connecting it to any non-local database.

## Rules

- Keep controllers thin.
- Place business use cases outside HTTP handlers.
- Do not put provider SDK calls directly in controllers.
- Persist operations before dispatching external side effects.
- Keep secrets out of logs and source control.
- Add migrations and tests with each domain feature.

## Status

The app scaffold has not yet been generated or compiled in this repository. The command above is the Phase 0 implementation entry point.