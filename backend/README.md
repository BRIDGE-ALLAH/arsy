# ARSY Backend

The backend is Rust. The first application is the Loco.rs Control Plane API.

## Pinned toolchain
- Rust 1.99.1 is pinned in the repository-level `rust-toolchain.toml`.
- Loco generator 1.3.1 and SeaORM CLI 2.0.0 are installed by `scripts/bootstrap-backend.sh`.
- PostgreSQL is the source of truth; the app uses Loco's PostgreSQL durable background queue.

## Generate the Control Plane

From the repository root run `bash scripts/bootstrap-backend.sh`. The script invokes the official Loco generator with explicit, non-interactive options for PostgreSQL persistence, PostgreSQL-backed jobs, and a headless API.

After generation, set the development database URI to match credentials created by `scripts/dev-up.sh`. Never copy development credentials into production.

## Rules
- Keep controllers thin and business use cases outside HTTP handlers.
- Do not call provider SDKs directly from controllers.
- Persist operations before external side effects.
- Keep secrets out of logs and source control.
- Add migrations and tests with each domain feature.

## Status

The pinned generator bootstrap is committed. The generated Loco application, compiled API, OpenAPI contract, and runtime health checks still require source generation, review, and verified tests; they are not marked complete yet.
