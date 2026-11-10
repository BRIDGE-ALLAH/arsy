# ARSY Development Guide

## Toolchain versions

The repository pins these contributor toolchains:

- Rust: stable channel, with `rustfmt` and `clippy` components.
- Node.js: 26.x Active LTS (see `.nvmrc` and `.node-version`).
- pnpm: exactly 12.19.0 via the root `package.json` `packageManager` field.

Install Rust with rustup, then let the repository toolchain file select the required channel and components:

```bash
rustup toolchain install stable --profile minimal --component rustfmt --component clippy
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
- Rust stable toolchain (for backend work)
- Node.js 26.x and pnpm 12.19.0 (for frontend work)

## 1. Clone and start PostgreSQL

```bash
git clone https://github.com/BRIDGE-ALLAH/arsy.git
cd arsy
./scripts/dev-up.sh
```

The script creates or repairs a local .env with random PostgreSQL and JWT secrets. Existing non-placeholder values are preserved. The file is ignored by Git and has mode 0600.

PostgreSQL is bound to 127.0.0.1:5432. The local database defaults to:

- Database: arsy
- User: arsy
- Host: 127.0.0.1
- Port: 5432

The database password and JWT secret are stored in the local .env file. Do not paste them into issues or logs.

## 2. Generate the Loco control plane

Follow the command in [backend/README.md](../backend/README.md). The official generator writes a standard Loco application under backend/apps/arsy-control-plane.

After generation:
- Review generated Cargo.toml and config files.
- Confirm the PostgreSQL connection uses local development values.
- Keep queue mode configured for PostgreSQL for durable job behavior.
- Commit Cargo.lock for the generated application.
- Add a health endpoint check and an initial migration before building domain modules.

## 3. Frontend

The initial frontend applications are planned under frontend/apps. Use Next.js App Router and TypeScript. Keep the API client separate from UI components and generate types from OpenAPI once the Control Plane contract stabilizes.

## 4. Stop local dependencies

```bash
./scripts/dev-down.sh
```

The PostgreSQL volume is preserved. To intentionally delete all local database data, use Docker Compose with the documented development compose file and the -v flag after confirming that the data is disposable.

## Local-only warning

These scripts are for local development only. They do not install ARSY in production, do not configure a firewall, do not publish services, and do not create a production administrator. Production onboarding is a Phase 2 deliverable; see [INSTALLER.md](INSTALLER.md).
