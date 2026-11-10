# ARSY Roadmap

This roadmap separates architecture preparation from working product capability. Dates are intentionally omitted until team capacity, provider access, and infrastructure budget are known.

## Phase 0 — Repository and development foundation

**Goal:** contributors can build and run the development dependencies consistently.

Deliverables:
- Repository conventions, docs, ADRs, and security baseline.
- Rust stable toolchain and Loco API scaffold.
- PostgreSQL development environment.
- Next.js console scaffold.
- CI checks for formatting, linting, tests, and frontend type checking.
- Local developer scripts.

Exit criteria:
- A clean checkout can start dependencies with one documented command.
- API starts and exposes health endpoints.
- Console starts and can reach a documented local API.
- Database migration command is documented and repeatable.
- CI runs on pull requests.

## Phase 1 — ARSY Core

**Goal:** dependable identity, tenancy, and resource records.

Deliverables:
- Authentication and session lifecycle.
- Organizations, memberships, roles, projects, environments.
- Provider account references and encrypted secret integration.
- Region and product catalog.
- Resource and operation registry.
- Durable PostgreSQL job queue.
- Audit trail and request correlation.
- API contract and first console navigation.

Exit criteria:
- Tenant isolation tests pass.
- Unauthorized operations are rejected and audited.
- Jobs survive API/worker restarts.
- API idempotency behavior is documented.

## Phase 2 — ARSY Compute and single-server onboarding

**Goal:** one useful compute lifecycle and an approachable self-hosted installation.

Deliverables:
- First real provider adapter or ARSY node backend.
- Agent identity, authenticated protocol, heartbeat, and capabilities.
- Instance create/read/start/stop/reboot/delete.
- Desired/observed state reconciliation.
- Operation progress in the console.
- Single-server production deployment topology.
- **ARSY one-command installer**, upgrade, backup, restore, and uninstall workflows.
- Signed or integrity-verified release artifacts.

Exit criteria:
- Fresh supported server can be installed using the documented guided installer.
- Installer preflight fails safely on unsupported or unhealthy hosts.
- Secrets are generated securely and stored with restrictive permissions.
- Interrupted install can be resumed or safely rolled back.
- Upgrade and backup/restore are exercised on a test server.
- A real compute operation completes through the API and UI.

## Phase 3 — ARSY Network

**Goal:** reliable networking primitives around compute.

Deliverables:
- DNS provider integration or authoritative DNS management plan.
- Reverse proxy and ingress.
- TLS certificate automation.
- Basic load balancing.
- Firewall integration and network policy model.

Exit criteria:
- Public ingress is provisioned, monitored, and recoverable.
- TLS renewal and proxy reload are tested.
- Abuse and rate-limiting controls are documented.

## Phase 4 — ARSY Storage and managed platform

**Goal:** persistent data and application lifecycle services.

Deliverables:
- Object storage integration with S3-compatible API where appropriate.
- Backup/snapshot orchestration.
- Managed database provisioning (initially one engine).
- Application build/deploy workflows.
- Metering and quotas.

Exit criteria:
- Backup restore is verified.
- Data deletion and retention semantics are documented.
- Usage records reconcile with source infrastructure.

## Phase 5 — Operations and commercial readiness

**Goal:** operational maturity for early customers.

Deliverables:
- Billing plans, invoices, and payment integration.
- Support tooling and audit exports.
- SLOs, alerting, incident runbooks.
- Capacity and cost reporting.
- Security review and penetration test scope.
- Release and upgrade policy.

Exit criteria:
- Incident, recovery, and customer support procedures have been rehearsed.
- Cost, quota, and billing discrepancies are observable.
- Security and privacy controls have an owner.

## Phase 6 — Regional cells

**Goal:** add a second region without coupling all runtime operations to one control plane process.

Deliverables:
- Regional cell architecture and local state ownership.
- Regional agent gateway and scheduler.
- Placement policy, data-residency controls, and regional capacity inventory.
- Cross-region health and failover procedures.

Exit criteria:
- A region can continue safe local operation during a control-plane connectivity interruption.
- Regional failure does not corrupt global resource inventory.
- Recovery and reconciliation are tested.

## Phase 7 — Global network and cloud ecosystem

**Goal:** expand into global cloud infrastructure capabilities.

Deliverables may include:
- Additional regions and providers.
- Global traffic management.
- CDN/cache infrastructure with real edge footprint.
- Advanced firewall and network security services.
- Multi-region storage and database patterns.
- Enterprise IAM, compliance, and support features.

Exit criteria:
- Each product has measurable coverage, capacity, security controls, and an operational owner.
- No global performance or availability claims exceed measured service capability.

## Installer placement

| Stage | What ships |
|---|---|
| Phase 0 | scripts/dev-up.sh and scripts/dev-down.sh for local development dependencies |
| Phase 1 | Bootstrap configuration conventions and environment validation |
| **Phase 2** | **Production one-command installer for a supported single Linux server**, plus upgrade/backup/restore/uninstall |

The Phase 2 installer is intentionally not shipped as a fake script before the production service images and runtime are real.