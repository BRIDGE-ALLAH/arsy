# ARSY

**Small Core. Global Infrastructure.**

ARSY is an independent hybrid cloud infrastructure platform designed to provision, operate, and scale infrastructure across ARSY-operated resources and supported third-party providers.

> **Project status:** architecture and foundation stage. ARSY is not production-ready and does not currently provide managed cloud services.

## Vision

Build an API-first, Rust-powered cloud platform with a modular control plane and distributed infrastructure agents. Start with a dependable single-region compute-management MVP, then expand toward networking, storage, managed databases, observability, billing, and global edge services.

## Architecture decisions

- **Control plane API:** Rust + Loco.rs.
- **Frontend:** Next.js App Router + TypeScript.
- **Database:** PostgreSQL.
- **Persistence and jobs:** Loco-supported PostgreSQL-backed background queue initially.
- **Infrastructure:** provider adapters, authenticated node agents, and explicit resource lifecycle reconciliation.
- **Deployment:** Docker Compose for development; a guarded single-server installer is a Phase 2 deliverable after runtime and agent contracts stabilize.
- **Architecture style:** modular monolith for the control plane, distributed agents and regional cells as the platform grows.

## Start local dependencies

Requirements: Docker Engine, Docker Compose v2, and OpenSSL.

~~~bash
git clone https://github.com/BRIDGE-ALLAH/arsy.git
cd arsy
./scripts/dev-up.sh
~~~

This starts PostgreSQL for local development only and creates a random local password in an ignored `.env` file. It does not install or start ARSY services.

Stop the dependency container without deleting its data:

~~~bash
./scripts/dev-down.sh
~~~

## Start the frontend

Requirements: Node.js 26 and pnpm 10.20.0.

~~~bash
cd frontend
pnpm install
pnpm dev:console
~~~

The public landing page runs with `pnpm dev:website`. These applications are scaffolds; authenticated cloud management is not available yet.

## Generate the backend

Install the pinned Rust toolchain using rustup, then run:

~~~bash
./scripts/bootstrap-backend.sh
~~~

The script generates the Loco Control Plane with PostgreSQL persistence and a PostgreSQL-backed worker queue. Review the generated configuration before connecting it to anything other than a local development database.

## Work tracking

The phased implementation backlog is maintained in [GitHub issue #1](https://github.com/BRIDGE-ALLAH/arsy/issues/1). Work one dependency-ready child issue per pull request.

## Project documents

- [Product narrative](docs/PRODUCT.md)
- [Master architecture blueprint](docs/MASTER_BLUEPRINT.md)
- [Development guide](docs/DEVELOPMENT.md)
- [Roadmap](docs/ROADMAP.md)
- [Installer and onboarding specification](docs/INSTALLER.md)
- [Security baseline](docs/SECURITY.md)
- [ADR-0001: Platform shape](docs/decisions/ADR-0001-platform-shape.md)

## Principles

1. Security by default and least privilege.
2. Infrastructure-agnostic provider contracts.
3. Idempotent provisioning and reconciliation.
4. Auditable operations and explicit tenant isolation.
5. Small, testable modules before distributed decomposition.
6. Honest capability boundaries: planned services are not advertised as available services.

## License

A license has not yet been selected. Do not assume this repository is licensed for redistribution until a LICENSE file is added.
