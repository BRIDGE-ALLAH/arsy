# ARSY

**Small Core. Global Infrastructure.**

ARSY is an independent hybrid cloud infrastructure platform designed to provision, operate, and scale infrastructure across ARSY-operated resources and supported third-party providers.

> Project status: architecture and foundation stage. ARSY is not yet production-ready and does not currently provide managed cloud services.

## Vision

Build an API-first, Rust-powered cloud platform with a modular control plane and distributed infrastructure agents. Start with a dependable single-region compute-management MVP, then expand toward networking, storage, managed databases, observability, billing, and global edge services.

## Architecture decisions

- **Control plane API:** Rust + Loco.rs.
- **Frontend:** Next.js App Router + TypeScript.
- **Database:** PostgreSQL.
- **Persistence and jobs:** Loco-supported persistent background jobs initially; introduce a distributed event broker only when cross-process or cross-region requirements justify it.
- **Infrastructure:** provider adapters, authenticated node agents, and explicit resource lifecycle reconciliation.
- **Deployment:** Docker Compose for development; a guarded single-server installer is a Phase 2 deliverable after the runtime and agent contracts stabilize.
- **Architecture style:** modular monolith for the control plane, distributed agents and regional cells as the platform grows.

## Repository layout

```text
arsy/
├── backend/       # Rust workspace and Loco.rs control plane
├── frontend/      # Next.js console and public website
├── deploy/        # Local development and deployment assets
├── scripts/       # Development and maintenance scripts
└── docs/          # Product, architecture, security, API, and operations docs
```

See [the master blueprint](docs/MASTER_BLUEPRINT.md), [the roadmap](docs/ROADMAP.md), and [the installer plan](docs/INSTALLER.md).

## Development status

The initial repository foundation is being established. Features must be considered incomplete until they have implementation, automated tests, operational documentation, and a verified deployment path.

## Principles

1. Security by default and least privilege.
2. Infrastructure-agnostic provider contracts.
3. Idempotent provisioning and reconciliation.
4. Auditable operations and explicit tenant isolation.
5. Small, testable modules before distributed decomposition.
6. Honest capability boundaries: planned services are not advertised as available services.

## License

License to be selected before public distribution. Do not assume an open-source license until a LICENSE file is added.
