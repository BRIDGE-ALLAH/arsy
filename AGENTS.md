# ARSY repository instructions

## Before changing code

1. Read `README.md`, `docs/PRODUCT.md`, `docs/MASTER_BLUEPRINT.md`, `docs/SECURITY.md`, and the relevant ADRs.
2. Read the complete GitHub issue, its stable task ID, acceptance criteria, and every declared dependency. Do not begin blocked work.
3. Work on one child issue per branch and pull request. Keep changes within that issue's scope.
4. Treat docs and issue acceptance criteria as the source of truth. Planned capabilities are not implemented capabilities.

## Repository map

- `backend/apps/arsy-control-plane`: Loco.rs HTTP Control Plane.
- `backend/apps/arsy-control-plane/migration`: SeaORM schema migrations.
- `frontend/apps/console`: Next.js App Router customer Console.
- `frontend/apps/website`: public website (separate scaffold task).
- `deploy/`: local development infrastructure and future deployment assets.
- `scripts/`: local development helpers. They are not production installers.
- `docs/`: product, architecture, security, operations, and decision records.

## Commands

Start local PostgreSQL and create a local-only secret file:

```bash
./scripts/dev-up.sh
```

Start the API (requires Rust and Cargo):

```bash
bash scripts/dev-api.sh
```

Backend checks (from the repository root):

```bash
cargo fmt --manifest-path backend/apps/arsy-control-plane/Cargo.toml --all -- --check
cargo clippy --manifest-path backend/apps/arsy-control-plane/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path backend/apps/arsy-control-plane/Cargo.toml
```

Frontend checks (requires Node 26 and pnpm 12.19.0; run after the root lockfile has been generated):

```bash
pnpm install --frozen-lockfile
pnpm lint:console
pnpm typecheck:console
pnpm build:console
```

## Architecture guardrails

- Keep the Control Plane a modular monolith until measurable requirements justify decomposition.
- Provider-specific behavior belongs behind capability-aware contracts/adapters.
- Separate desired state from observed state. Persist operations before external side effects; make them idempotent and reconcilable.
- Keep HTTP handlers thin and domain behavior in the Loco model layer.
- Keep tenants isolated at every query, resource lookup, job, and API boundary.
- Backend authorization is authoritative. UI hiding is not authorization.
- Keep credentials server-side, encrypted at rest where persisted, and redacted in logs/errors.
- Agents are privileged infrastructure components: authenticated machine identity, revocation, least privilege, typed allowlisted commands, no arbitrary shell execution.
- Never add a public admin default password, first-claim race, unauthenticated privileged endpoint, or unsafe install path.
- Never expose database connection strings, JWT secrets, provider tokens, request credentials, or raw dependency errors.
- Do not claim production readiness, a service-level objective, certification, provider support, region, POP, or product availability without evidence.

## Change and verification rules

- Add tests alongside behavior. Prefer request/integration tests for HTTP behavior and model tests for domain rules.
- Add a migration whenever persisted schema changes; do not edit generated SeaORM entity files by hand.
- Run formatting, linting, type checking, builds, and relevant tests. Report commands that could not be run and why.
- Never mark an acceptance item complete without evidence. Do not mark an issue done when required external credentials, real infrastructure, or acceptance tests are unavailable.
- Do not commit `.env`, private keys, certificates, provider credentials, customer data, build outputs, or generated secret material.
- Do not silently broaden issue scope. If an ADR or API contract must change, update it in the designated issue.

## Pull request expectations

Use the repository pull request template. Include the stable task ID (for example `P1-09`) in the title and body, link the issue, list changed files and exact test results, identify unverified acceptance items, and state any external prerequisites. Keep PRs reviewable and do not bundle unrelated backlog items.
