# ARSY Development Guide

## Prerequisites

- Git
- Docker Engine and Docker Compose v2
- OpenSSL
- Rust stable toolchain (for backend work)
- Node.js LTS and pnpm (for frontend work)

## 1. Clone and start PostgreSQL

~~~bash
git clone https://github.com/BRIDGE-ALLAH/arsy.git
cd arsy
./scripts/dev-up.sh
~~~

The script creates a local .env with a random PostgreSQL password if one does not already exist. The file is ignored by Git and created with restrictive permissions.

PostgreSQL is bound to 127.0.0.1:5432. The local database defaults to:

- Database: arsy
- User: arsy
- Host: 127.0.0.1
- Port: 5432

The password is stored in the local .env file. Do not paste it into issues or logs.

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

~~~bash
./scripts/dev-down.sh
~~~

The PostgreSQL volume is preserved. To intentionally delete all local database data, use Docker Compose with the documented development compose file and the -v flag after confirming that the data is disposable.

## Local-only warning

These scripts are for local development only. They do not install ARSY in production, do not configure a firewall, do not publish services, and do not create a production administrator. Production onboarding is a Phase 2 deliverable; see [INSTALLER.md](INSTALLER.md).