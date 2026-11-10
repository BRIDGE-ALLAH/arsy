# ARSY Backend

The backend is a Cargo workspace. The first member is the Loco.rs PostgreSQL Control Plane.

## Workspace layout

- `Cargo.toml`: workspace root and shared dependency declarations.
- `Cargo.lock`: the single lockfile for every backend workspace member.
- `apps/arsy-control-plane`: Loco HTTP API and PostgreSQL-backed worker.
- `apps/arsy-control-plane/migration`: SeaORM migrations, included by the app.

## Common commands

Run these commands from the repository root:

```bash
cargo metadata --manifest-path backend/Cargo.toml --format-version 1 --no-deps
cargo fmt --manifest-path backend/Cargo.toml --all -- --check
cargo check --manifest-path backend/Cargo.toml --workspace
cargo test --manifest-path backend/Cargo.toml --workspace
```

Run the API from the application directory after starting PostgreSQL and exporting the local development environment:

```bash
cd backend/apps/arsy-control-plane
cargo loco start
```

See [Development Guide](../docs/DEVELOPMENT.md) for environment setup.

## Rules

- Keep controllers thin and domain behavior on Loco models.
- Do not put provider SDK calls directly in controllers.
- Persist operations before dispatching external side effects.
- Keep secrets out of logs and source control.
- Add migrations and tests with each domain feature.
