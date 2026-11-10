# ARSY Development Guide

## Toolchain versions

The repository pins these contributor toolchains:

- Rust: exactly `1.99.1`, with `rustfmt` and `clippy` components (`rust-toolchain.toml`).
- Node.js: 26.x Active LTS (see `.node-version` and `frontend/.nvmrc`).
- pnpm: exactly 12.19.0 via the root `package.json` `packageManager` field.

Install Rust with rustup, then let the repository toolchain file select the required channel and components:

```bash
rustup toolchain install 1.99.1 --profile minimal --component rustfmt --component clippy
rustc --version
cargo --version
```

Install Node.js 26 using your preferred version manager (for example, `nvm install` reads `.nvmrc`), then activate the pinned package manager. On machines with Corepack installed:

```bash
corepack enable pnpm
pnpm --version
```

If Corepack is not installed with your Node distribution, install/enable Corepack using its official instructions, or install the exact package manager version directly:

```bash
npm install --global pnpm@12.19.0
pnpm --version
```

The expected pnpm version is `12.19.0`. Do not replace the exact root `packageManager` pin with `latest` or a floating range.

## Prerequisites

- Git
- Docker Engine and Docker Compose v2
- OpenSSL
- Rust 1.99.1 toolchain (for backend work)
- Node.js 26.x and pnpm 12.19.0 (for frontend work)

## 1. Clone and start PostgreSQL

```bash
git clone https://github.com/BRIDGE-ALLAH/arsy.git
cd arsy
./scripts/dev-up.sh
```

The script creates a local `.env` with a random PostgreSQL password and JWT signing secret if needed. The file is ignored by Git and created with restrictive permissions.

PostgreSQL is bound to 127.0.0.1:5432. The local database defaults to:

- Database: arsy
- User: arsy
- Host: 127.0.0.1
- Port: 5432

The password is stored in the local .env file. Do not paste it into issues or logs.

## 2. Start the Loco control plane

The generated application is committed under `backend/apps/arsy-control-plane`. Start the local PostgreSQL dependency and launch the API with:

```bash
bash scripts/dev-backend.sh
```

The launcher reads only the expected URL-safe variables from the ignored local `.env` file and exports `DATABASE_URL`, `QUEUE_URL`, and `JWT_SECRET` for the process. Development config has no fallback database URI or JWT signing secret. The development mailer is stubbed so local auth flows do not require an SMTP server.

The built-in monitoring endpoints are:

- `GET /_ping` — process liveness.
- `GET /_health` — application health.
- `GET /_readiness` — readiness probe.

All three are mounted by Loco's default routes. The readiness handler is framework-owned; do not replace it with a self-reported static response.

## 3. Frontend

The Next.js App Router applications are under `frontend/apps/console` and `frontend/apps/website`. Use `pnpm --filter @arsy/console dev` or `pnpm --filter @arsy/website dev` from the `frontend` workspace. Keep API clients separate from UI components and generate types from the versioned OpenAPI contract.

## 4. Stop local dependencies

```bash
./scripts/dev-down.sh
```

The PostgreSQL volume is preserved. To intentionally delete all local database data, use Docker Compose with the documented development compose file and the -v flag after confirming that the data is disposable.

## Local-only warning

These scripts are for local development only. They do not install ARSY in production, do not configure a firewall, do not publish services, and do not create a production administrator. Production onboarding is a Phase 2 deliverable; see [INSTALLER.md](INSTALLER.md).
