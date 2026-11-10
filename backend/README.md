# ARSY Backend

The backend contains the Loco.rs PostgreSQL Control Plane application under `apps/arsy-control-plane`.

## Run locally

From the repository root:

```bash
./scripts/dev-up.sh
bash scripts/dev-api.sh
```

The development launcher reads the local ignored `.env`, constructs the PostgreSQL connection URLs, and exports a locally generated JWT secret. Do not use this launcher or its credentials in production.

## Common commands

Run from `backend/apps/arsy-control-plane`:

```bash
cargo loco start
cargo loco routes
cargo loco db migrate
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## Rules

- Keep controllers thin and domain behavior on Loco models.
- Do not put provider SDK calls directly in controllers.
- Persist operations before dispatching external side effects.
- Keep secrets out of logs and source control.
- Add migrations and tests with each domain feature.
