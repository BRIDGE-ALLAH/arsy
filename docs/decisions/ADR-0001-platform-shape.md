# ADR-0001: ARSY platform shape

- **Status:** Accepted
- **Date:** 2026-11-10

## Context

ARSY aims to become a global hybrid cloud infrastructure platform while beginning with a small engineering team and an initially limited provider footprint.

## Decision

1. Build the first Control Plane as a Rust + Loco.rs modular monolith.
2. Use PostgreSQL as the system of record and as the initial durable background queue.
3. Use Next.js App Router + TypeScript for the customer console and public website.
4. Keep provider-specific infrastructure behind capability-aware adapters.
5. Add a narrowly scoped node agent for ARSY-operated infrastructure when compute provisioning begins.
6. Use Docker Compose for development; ship a production one-command installer in Phase 2 only after a coherent runtime release exists.
7. Introduce distributed messaging, regional cells, and service extraction only when justified by measured requirements.

## Consequences

- The initial system is simpler to develop and operate.
- Domain boundaries must remain explicit to avoid a tightly coupled monolith.
- PostgreSQL queue throughput and failure handling must be measured before large-scale orchestration.
- Regional autonomy and global data residency require additional architecture work before multi-region production.
- The one-command installer is a product requirement, but a premature installer would create a false impression of production readiness.