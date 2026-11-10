# ARSY Security Baseline

Security work is required from the first API and remains a release gate for every infrastructure feature.

## Identity and tenancy

- Enforce authentication for all non-public control-plane endpoints.
- Enforce organization/project ownership on every resource access.
- Use least-privilege roles and scoped service identities.
- Protect admin bootstrap; never allow unauthenticated first-visitor admin claims on a public interface.
- Add MFA support before production multi-user exposure.

## Secrets and credentials

- Never commit .env, provider tokens, certificates, private keys, or production connection strings.
- Encrypt provider credentials at rest or store references to an external secret manager.
- Keep encryption keys separate from the encrypted data and back them up securely.
- Redact secrets from structured logs, errors, traces, and API responses.
- Do not pass secrets in command-line arguments.

## Agent and infrastructure safety

- Authenticate both ends of agent communication.
- Use revocable node identity and short-lived credentials where feasible.
- Accept typed, allowlisted commands only; never expose arbitrary shell execution.
- Run the agent with minimum operating-system privileges.
- Validate command parameters and bound resource usage.
- Authenticate and integrity-check agent updates.
- Record actor, target, command, result, timestamp, and correlation ID for privileged operations.

## Provisioning and reliability

- Require idempotency for retryable create operations.
- Persist operations before dispatching external side effects.
- Distinguish accepted, running, succeeded, and failed operations.
- Reconcile desired and observed state.
- Apply bounded retries and backoff; expose terminal failures for operator action.
- Never delete user resources during an automatic rollback without explicit, documented semantics.

## Network and deployment

- Keep databases and internal service ports private.
- Restrict inbound ports to the minimum needed.
- Do not disable firewalls globally.
- Use TLS for public and service-to-service traffic.
- Verify release artifacts and pin container images by version/digest.
- Keep backups encrypted and test restores.

## Release gates

Before production:
- Tenant isolation tests.
- Authorization matrix tests.
- Secret redaction tests.
- Dependency and container scanning.
- Threat model for API, worker, agent, and installer.
- Backup and restore rehearsal.
- Upgrade and rollback rehearsal.
- Incident and credential-rotation runbooks.