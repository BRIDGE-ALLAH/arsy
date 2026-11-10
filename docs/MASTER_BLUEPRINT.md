# ARSY Master Blueprint

**Version:** 1.0  
**Status:** Architecture baseline  
**Product direction:** Hybrid cloud, global ambition  
**Tagline:** Small Core. Global Infrastructure.

## 1. Executive summary

ARSY is an independent hybrid cloud infrastructure platform. One control plane provides a consistent API and console while infrastructure may live on ARSY-operated hardware, external VPS vendors, or public cloud providers.

The architecture is deliberately incremental:

- A **modular monolith** for the initial control plane.
- Separate **workers** for asynchronous provisioning and reconciliation where needed.
- A small, authenticated **node agent** for ARSY-operated infrastructure.
- Provider adapters for external infrastructure.
- **Regional cells** as a future scale and fault-containment boundary.
- A distinct **edge plane** only when networking products require it.

Do not begin with dozens of microservices. Extract a service only when independent scaling, isolation, deployment cadence, or team ownership justify the operational cost.

## 2. Official product narrative

ARSY is an independent cloud infrastructure platform built to simplify how developers and businesses provision, operate, and scale digital infrastructure.

Powered by Rust, ARSY combines infrastructure management, compute orchestration, networking, storage, and developer services in one coherent platform. Its hybrid architecture is designed to operate across ARSY-managed infrastructure and supported third-party providers.

ARSY starts with a lightweight, secure core and grows toward a global cloud ecosystem. The platform prioritizes API-first workflows, automation, clear resource lifecycle management, tenant isolation, and operational resilience.

**Small Core. Global Infrastructure.**

## 3. Architecture at a glance

~~~mermaid
flowchart TB
  U[Users / CLI / SDK] --> F[Next.js Console]
  U --> API[ARSY Control Plane API]
  F --> API
  API --> IAM[Identity & Tenant]
  API --> DOM[Domain Modules]
  DOM --> DB[(PostgreSQL)]
  DOM --> Q[Persistent Job Queue]
  Q --> W[Worker / Reconciler]
  W --> PA[Provider Adapters]
  W --> AG[ARSY Node Agent]
  PA --> EXT[External VPS / Public Cloud]
  AG --> NODES[ARSY-managed Nodes]
  API --> OBS[Telemetry & Audit]
~~~

### Control Plane

The Control Plane is the system of record for identity, tenancy, resource intent, provider configuration, operation state, audit, and billing metadata.

Responsibilities:

- Authentication, sessions, MFA integration, and authorization.
- Organizations, memberships, teams, projects, and environments.
- Product catalog, regions, provider accounts, and resource types.
- Resource desired state and observed state.
- Provisioning requests, operation history, retries, and reconciliation.
- Audit events, quotas, usage records, and billing metadata.
- API contracts consumed by the console, CLI, SDKs, and agents.

**Initial implementation:** Rust + Loco.rs, PostgreSQL, SeaORM, Tokio.

### Infrastructure Plane

The Infrastructure Plane executes work and hosts customer workloads.

It includes:

- ARSY-operated bare-metal and virtualization nodes.
- External VPS and dedicated-server accounts.
- Public-cloud resources reached through provider APIs.
- Container and VM runtimes.
- Storage and network components.

The control plane must not assume it owns every host or has direct shell access to every provider.

### Edge Plane

The Edge Plane is a future global networking layer for DNS, reverse proxy, TLS, load balancing, caching, traffic policy, and edge security integrations.

Do not market global CDN or DDoS mitigation until ARSY has the network capacity, upstream relationships, operational controls, and measured coverage to provide them.

## 4. Service boundaries

### Initial control-plane modules

These are domain modules inside one Loco application at first:

| Module | Responsibility |
|---|---|
| identity | Users, credentials, sessions, MFA hooks |
| tenancy | Organizations, memberships, teams, roles |
| projects | Projects and environments |
| catalog | Product, region, size, and image catalog |
| providers | Provider accounts, credential references, adapters |
| resources | Resource registry, desired/observed state |
| provisioning | Operation requests, idempotency, status |
| orchestration | Scheduling, placement policy, reconciliation |
| audit | Append-only business and administrative audit events |
| metering | Usage observations and aggregation |
| billing | Plans, quotas, invoices (later phase) |
| notifications | Email and system notifications |

### Runtime and infrastructure executables

Split these into separate binaries when their runtime or security boundary requires it:

| Executable | Responsibility | Delivery |
|---|---|---|
| arsy-control-plane | HTTP API and domain workflows | Phase 0–1 |
| arsy-worker | Persistent background jobs and reconciliation | Phase 1–2 |
| arsy-agent | Authenticated node-side operations | Phase 2 |
| arsy-cli | Operator and developer CLI | Phase 2+ |
| arsy-edge | Edge routing/data plane services | Phase 4+ |

A module is not automatically a microservice. Keep module APIs explicit so later extraction is possible.

## 5. Rust backend architecture

Recommended dependency direction:

~~~
HTTP / CLI / Agent adapters
          |
          v
Application use cases
          |
          v
Domain model and policies
          |
          v
Ports (provider, repository, queue, clock)
          ^
          |
Infrastructure adapters
(PostgreSQL, providers, runtime, telemetry)
~~~

Rules:

- Domain code must not depend on Loco controllers or a specific cloud SDK.
- Controllers validate and translate HTTP requests; they do not contain provisioning logic.
- Provider SDKs live behind adapter crates/modules.
- Use typed IDs and explicit domain errors. The canonical format and categories are defined in [Domain Primitives](DOMAIN.md).
- External side effects must be represented as durable operations.
- Commands must be idempotent or carry an idempotency key.
- Store desired state separately from observed state.
- Reconciliation must be safe to run more than once.
- Never execute user-provided shell strings on an agent.

### Suggested backend layout

Loco's generator owns the initial app skeleton. Keep the generated application conventional first; introduce workspace crates as code and ownership boundaries become real.

~~~
backend/
├── apps/
│   ├── arsy-control-plane/   # Loco application (Phase 0)
│   ├── arsy-agent/           # node agent (Phase 2)
│   └── arsy-cli/             # CLI (Phase 2+)
├── crates/
│   ├── arsy-domain/
│   ├── arsy-application/
│   ├── arsy-ports/
│   ├── arsy-provider-api/
│   ├── arsy-protocol/
│   ├── arsy-crypto/
│   └── arsy-telemetry/
└── README.md
~~~

Do not add empty crates purely to match a diagram. Add each crate when it has stable code to own.

### Loco application setup

Use the official Loco generator so its runtime, migrations, generated CLI, feature flags, and framework version stay internally consistent.

Suggested initial command from the repository root:

~~~bash
cargo install loco
cargo install sea-orm-cli
loco new \
  --path backend/apps/arsy-control-plane \
  --name arsy_control_plane \
  --db postgres \
  --bg queue-postgres \
  --assets none \
  --allow-in-git-repo
~~~

Review generated files and pin the exact generated framework/dependency versions in the committed lockfile. The app must use a PostgreSQL-backed queue for durable provisioning jobs; in-process async mode is suitable for simple local experiments, not durable infrastructure operations.

## 6. Frontend architecture

**Primary stack:** Next.js App Router + TypeScript.

Supporting stack:

- Tailwind CSS for styling.
- shadcn/ui for composable primitives.
- TanStack Query for remote/server state.
- Zustand only for genuinely client-owned state.
- React Hook Form + Zod for forms and validation.
- Apache ECharts for metrics and time-series charts.
- Lucide for icons.
- pnpm for package management; Turborepo only when shared packages and multiple apps justify it.

### Suggested frontend layout

~~~
frontend/
├── apps/
│   ├── console/       # authenticated cloud console
│   └── website/       # public marketing site
└── packages/
    ├── ui/
    ├── api-client/
    ├── types/
    └── config/
~~~

The console consumes the documented Control Plane API. Use generated OpenAPI types/client when the API surface becomes stable. Do not duplicate business rules in the browser.

## 7. Provider abstraction

Define provider capabilities explicitly. A provider must declare which operations and resource types it supports; never assume every provider implements the same lifecycle.

Conceptual interface:

~~~rust
#[async_trait::async_trait]
pub trait ComputeProvider: Send + Sync {
    async fn capabilities(&self) -> Result<ProviderCapabilities, ProviderError>;

    async fn create_instance(
        &self,
        request: CreateInstanceRequest,
        idempotency_key: IdempotencyKey,
    ) -> Result<ProviderOperation, ProviderError>;

    async fn get_instance(
        &self,
        external_id: &str,
    ) -> Result<ObservedInstance, ProviderError>;

    async fn start_instance(&self, external_id: &str)
        -> Result<ProviderOperation, ProviderError>;

    async fn stop_instance(&self, external_id: &str)
        -> Result<ProviderOperation, ProviderError>;

    async fn delete_instance(&self, external_id: &str)
        -> Result<ProviderOperation, ProviderError>;
}
~~~

Implementation notes:

- Use one adapter per provider and service family.
- Keep vendor-specific fields in adapter metadata, not the shared domain model.
- Normalize errors without erasing useful provider codes.
- Track external IDs, region, account reference, capabilities, and adapter version.
- Encrypt credentials using a managed key or envelope-encryption design; never store plaintext credentials.
- First adapter should be chosen based on a real test account and a complete create/read/stop/delete lifecycle.

Candidate integrations: ARSY node agent, one VPS provider, then one major public cloud. Do not build several shallow adapters before one adapter is reliable.

## 8. Resource lifecycle and operations

Resource status must distinguish intent from reality.

~~~mermaid
stateDiagram-v2
  [*] --> Requested
  Requested --> Provisioning
  Provisioning --> Active
  Provisioning --> Failed
  Active --> Updating
  Updating --> Active
  Updating --> Degraded
  Active --> Stopping
  Stopping --> Stopped
  Stopped --> Starting
  Starting --> Active
  Active --> Deleting
  Stopped --> Deleting
  Deleting --> Deleted
  Failed --> Provisioning: retry / reconcile
  Degraded --> Active: reconcile
  Degraded --> Failed: terminal failure
~~~

Store at least:

- Desired state and observed state.
- Status and status reason.
- Provider and provider account reference.
- Region and external resource ID.
- Idempotency key and operation ID.
- Last successful reconciliation time.
- Last error category and safe diagnostic details.
- Created/updated timestamps and tenant ownership.

Use transactional outbox or an equivalent durable handoff when a database write must reliably enqueue work. Worker retries need backoff, bounded attempts, failed states, and operator-visible recovery. A successful HTTP response means the operation was accepted, not that the instance is ready.

## 9. Data model baseline

Initial entities:

- users
- organizations
- memberships
- teams
- roles and permissions
- projects
- environments
- regions
- providers
- provider_accounts
- provider_credentials (encrypted or secret-manager references only)
- resource_types
- resources
- resource_operations
- nodes
- node_agents
- audit_events
- usage_records
- api_tokens

Billing entities such as plans, subscriptions, invoices, and payment records are introduced only when the billing phase begins.

Tenant isolation must be enforced in application queries and authorization policies, not only by hiding records in the UI. Add database-level constraints and row-level security only after testing the full connection-pool and tenant-context model.

## 10. Node agent and regional architecture

The node agent is a privileged component and must have a narrow, typed protocol.

Minimum requirements:

- Outbound authenticated connection to the control plane or regional gateway.
- Unique node identity and revocable credentials.
- mTLS or equivalent mutually authenticated transport.
- Command allowlist with typed arguments; never arbitrary shell execution.
- Signed or authenticated software updates.
- Heartbeat, capacity report, and version/capability report.
- Durable local operation journal for restart recovery.
- Audit correlation IDs for every privileged action.
- Least-privilege operating-system user and capabilities.

Future regional-cell shape:

~~~mermaid
flowchart TB
  G[Global Management Plane] --> R1[Jakarta Cell]
  G --> R2[Singapore Cell]
  G --> R3[Other Regional Cells]
  R1 --> N1[Regional nodes and services]
  R2 --> N2[Regional nodes and services]
  R3 --> N3[Regional nodes and services]
  G --> EXT[External provider APIs]
~~~

Each cell should eventually own local scheduling, local operations, and a local failure boundary. The global plane coordinates identity, product catalog, global resource inventory, and policy. Define data residency, regional failover, and tenant placement policy before multi-region production rollout.

## 11. Networking, storage, and edge strategy

- Start with provider-managed networking and a simple ingress/reverse-proxy layer.
- Add DNS management and certificate automation after compute lifecycle is reliable.
- Integrate object storage before attempting to operate a globally distributed storage system.
- Treat block storage, snapshots, and backup consistency as separate engineering work.
- Build CDN and edge routing only with a measured edge footprint, traffic capacity, origin protection, abuse controls, and operational ownership.

## 12. Observability and operations

Required from the beginning:

- Structured logs with request, tenant, resource, and operation correlation IDs.
- Metrics for API latency/errors, queue depth, job age, reconciliation failures, node heartbeat, and provider API failures.
- Health, readiness, and liveness endpoints.
- Traces across API → queue → worker → provider adapter.
- Audit records for privileged actions.
- Backup/restore drills for PostgreSQL and application secrets.
- Documented upgrade and rollback procedure.

OpenTelemetry instrumentation should be added incrementally. Prometheus-compatible metrics are a good initial operational interface.

## 13. Initial deployment model

Development:

- Docker Compose for PostgreSQL and local dependencies.
- Loco app runs locally with Cargo for fast feedback.
- Next.js console runs locally with pnpm.
- No production installer until a coherent service image, persistent volumes, secrets, upgrade path, and health checks exist.

First self-hosted production edition:

- One supported Linux server.
- Control plane, worker, and database with persistent volumes.
- HTTPS and a supported reverse proxy.
- Automated secret generation and restrictive file permissions.
- Admin account bootstrap that cannot be claimed by an arbitrary internet visitor.
- Backup, upgrade, restore, and uninstall workflows.

The installer is a Phase 2 deliverable, described in INSTALLER.md.

## 14. Architecture guardrails

1. No customer workload runs with control-plane credentials.
2. No provider secret is returned to the frontend after creation.
3. No unauthenticated agent command channel.
4. No arbitrary shell command API.
5. No cross-tenant resource lookup without an authorization decision.
6. No non-idempotent automatic retry for create/delete operations.
7. No claim of global availability, CDN, or DDoS protection before capacity and coverage are real.
8. No microservice extraction without an operational reason and an owner.
9. Every production migration needs a rollback or forward-recovery plan.
10. Every feature needs tests, observability, documentation, and a threat review proportional to risk.

## 15. Definition of a credible MVP

The compute-management MVP is credible only when it can:

- Authenticate an operator and enforce organization/project ownership.
- Register one real provider or node type.
- Provision one supported instance type end to end.
- Expose operation progress and observed state.
- Safely start, stop, reboot, and delete.
- Survive worker restart without losing jobs.
- Reconcile drift and recover failed operations.
- Record audit events.
- Enforce least privilege and protect provider credentials.
- Pass integration tests against a real or controlled provider sandbox.

A dashboard mockup or a successful API response alone is not a cloud platform.