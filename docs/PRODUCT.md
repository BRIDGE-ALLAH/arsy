# ARSY Product Narrative

**ARSY — Small Core. Global Infrastructure.**

ARSY is an independent hybrid cloud infrastructure platform designed to make infrastructure easier to provision, operate, and scale.

Powered by Rust, ARSY brings compute, networking, storage, database, security, observability, and developer services into one coherent platform. Its hybrid architecture can operate across ARSY-owned infrastructure and supported third-party providers without making the product dependent on a single vendor.

ARSY begins with a focused compute-management product and grows into a broader cloud ecosystem. The long-term ambition is global, but every release must deliver a real, secure, testable capability before the next layer is added.

## Positioning

ARSY is not a Coolify clone. Coolify is a useful reference for approachable self-hosted onboarding and deployment UX. ARSY's product ambition is broader: a cloud control plane that can manage infrastructure across multiple providers and, over time, ARSY-operated regional capacity.

## Product principles

1. **Simple onboarding:** an operator should be able to install a supported single-node edition with a guided, documented workflow.
2. **Hybrid by design:** provider adapters and ARSY-native infrastructure share stable domain contracts.
3. **Rust-first:** performance and memory safety matter, but operational correctness matters more.
4. **API-first:** every user-facing capability must be automatable through documented APIs.
5. **Secure by default:** tenant isolation, least privilege, secret handling, auditability, and safe defaults are core constraints.
6. **Incremental scale:** start with one dependable region and evolve toward regional cells and global services.

## Product families (long-term vision)

- **ARSY Compute:** virtual machines, containers, bare metal, and application workloads.
- **ARSY Network:** DNS, routing, load balancing, reverse proxy, TLS, and firewall integrations.
- **ARSY Storage:** object and block storage, snapshots, and backup orchestration.
- **ARSY Database:** managed PostgreSQL, MySQL, and Redis-compatible services.
- **ARSY Security:** identity, access policies, secrets, certificates, and audit.
- **ARSY Observability:** metrics, logs, traces, alerting, and health.
- **ARSY Developer Platform:** API, CLI, SDKs, build and deployment workflows.
- **ARSY Billing:** usage metering, plans, quotas, invoices, and cost allocation.
- **ARSY Edge:** global traffic management, caching, and edge services.

These are product goals, not a claim that these services are currently available.

## First release

The first useful release is **ARSY Compute Management**:

- Register an infrastructure provider or supported node.
- Discover and register compute capacity.
- Create and track a compute instance through a provider adapter.
- Start, stop, reboot, and delete an instance.
- Reconcile observed provider state with desired state.
- Show operation history and audit records.

The first release must be honest about which provider adapters and actions are implemented.