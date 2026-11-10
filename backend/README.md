# ARSY Backend

The backend is a Cargo workspace. Its first application is the Loco.rs Control Plane API at `apps/arsy-control-plane`.

## Pinned toolchain
- Rust 1.99.1 is pinned in the repository-level `rust-toolchain.toml`.
- Loco generator 1.3.1 and SeaORM CLI 2.0.0 are installed by `scripts/bootstrap-backend.sh`.
- PostgreSQL is the source of truth; the app uses Loco's PostgreSQL durable background queue.
- The single workspace lockfile is `backend/Cargo.lock`.

## Generate the Control Plane

From the repository root run `bash scripts/bootstrap-backend.sh`. The script invokes the official Loco generator with explicit, non-interactive options for PostgreSQL persistence, PostgreSQL-backed jobs, and a headless API.

After generation, set the development database URI to match the credentials created by `scripts/dev-up.sh`. Never copy development credentials into production.

## Workspace commands

Run from the repository root:

~~~bash
cargo metadata --manifest-path backend/Cargo.toml --no-deps
cargo fmt --manifest-path backend/Cargo.toml --all -- --check
cargo check --locked --workspace --manifest-path backend/Cargo.toml
cargo clippy --locked --workspace --manifest-path backend/Cargo.toml --all-targets -- -D warnings
cargo test --locked --workspace --manifest-path backend/Cargo.toml
~~~

The generated Loco app remains a normal workspace member; framework dependency choices are preserved in `backend/Cargo.toml`.

## Local development

Start PostgreSQL and the local API with `bash scripts/dev-backend.sh`. The launcher reads URL-safe credentials from the ignored root `.env`, then exports database, queue, and JWT configuration for the process. Never log or share that file.

## Rules
- Keep controllers thin and business use cases outside HTTP handlers.
- Do not call provider SDKs directly from controllers.
- Persist operations before external side effects.
- Keep secrets out of logs and source control.
- Add migrations and tests with each domain feature.

The Loco application and workspace compile in CI. Authentication and product-domain services remain under active development; this is not a production cloud service.
