# ARSY Repository Agent Guide

## Mission
Build ARSY as an independent hybrid cloud platform: **Small Core. Global Infrastructure.** The control plane coordinates ARSY-operated infrastructure and supported third-party providers through capability-aware adapters.

## Read before changing code
1. `README.md`
2. `docs/PRODUCT.md`
3. `docs/MASTER_BLUEPRINT.md`
4. `docs/SECURITY.md`
5. `docs/DEVELOPMENT.md`
6. The relevant ADR and GitHub child issue.

## Architecture guardrails
- Start with a modular monolith control plane, distributed infrastructure agents, and explicit provider ports. Do not split services prematurely.
- Keep domain logic independent of provider-specific SDKs and resource naming. Model provider capabilities explicitly.
- Separate desired state from observed state. Provisioning must be durable, idempotent, observable, retry-bounded, and reconciled.
- PostgreSQL is the initial system of record and durable job substrate. Introduce additional infrastructure only with an ADR and demonstrated need.
- Keep tenant, organization, project, and environment boundaries explicit in persistence and authorization.
- Use typed IDs, domain errors, validated inputs, and versioned external protocols.
- Do not expose arbitrary shell execution, unrestricted host access, or public administrative bootstrap endpoints.

## Security requirements
- Default to deny and least privilege. Enforce authorization on the server for every resource operation.
- Never log credentials, tokens, private keys, or secret values. Store secrets encrypted and expose references, not plaintext, in ordinary APIs.
- Authenticate agents with revocable machine identity; prefer mutually authenticated transport.
- Validate tenant ownership in database queries and service boundaries; add regression tests for cross-tenant access.
- Use parameterized database operations and bounded resource consumption.
- Treat provider responses, webhooks, uploaded artifacts, and agent messages as untrusted input.
- Avoid destructive defaults. Require explicit confirmation/authorization for irreversible operations.

## Engineering workflow
- Work one dependency-ready child issue per pull request. Do not close an issue until its acceptance criteria are verified.
- Keep changes small, typed, documented, and testable. Do not claim tests passed unless they were run and their output was inspected.
- Add unit tests for domain behavior, integration tests for persistence/authorization, and contract tests for provider/agent boundaries.
- Update OpenAPI, API docs, and examples whenever public API contracts change.
- Pin toolchains and dependencies; commit lockfiles. Avoid floating production image tags.
- Never commit secrets, local `.env` files, credentials, generated private keys, or customer data.
- Record consequential architecture choices as ADRs in `docs/decisions/`.
- If a requirement is ambiguous, preserve the established product/security constraints and document the assumption in the PR.

## Definition of done
- The linked issue's acceptance criteria are met.
- Formatting, linting, type checks, and relevant tests pass.
- Security and tenant-isolation implications have been reviewed.
- Docs/configuration/migrations are included where needed.
- CI is green, and the PR states exactly what was and was not verified.
