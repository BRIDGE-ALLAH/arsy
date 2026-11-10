# ARSY API Contract

## Versioning policy

All ARSY-owned REST API routes live under `/api/v1`. The OpenAPI root declares `security: []` because the currently implemented liveness, readiness, and account-authentication routes are public; the authenticated current-user operation explicitly overrides this with bearer JWT security. Compatible additive changes may be made within a version. A breaking change requires a new major path prefix (for example, `/api/v2`) with an explicit migration/deprecation plan. Framework-owned operational probes such as Loco's `/_ping`, `/_health`, and `/_readiness` stay outside the application API version prefix.

The first application routes are mounted at:

- `/api/v1/auth/*`: account registration, verification, login, logout, password recovery, magic links, and current-user handlers. Login and magic-link exchange issue JWTs backed by persisted sessions.
- `/api/v1/health/ready`: readiness for the required PostgreSQL dependency.
- `/_ping`: Loco's built-in process liveness response (`{"ok":true}`).

## Authentication sessions

Every issued JWT carries a random `sid` claim and has a matching PostgreSQL `auth_sessions` row. The token is returned only after the session row has been persisted. Protected handlers must use Loco's JWT extractor and then call `crate::auth::session::require_active_session`; a valid signature alone is not sufficient. Logout revokes only the presented session. Password reset revokes all active sessions before changing the password. Missing, expired, revoked, and unknown sessions fail closed with HTTP 401.

The first release invalidates older stateless JWTs that do not contain a persisted `sid`. Clients must sign in again after deployment rather than retaining credentials that cannot be revoked server-side.

## OpenAPI source and artifact

- Canonical source: `docs/api/source/openapi.yaml`.
- Bundled/reference artifact: `docs/api/openapi.yaml`.
- Generator: `scripts/generate-openapi.sh` copies the source deterministically. When local references are introduced, this script is the single place to replace the copy step with a pinned bundler invocation.
- Validation: Redocly CLI is pinned to an exact version in the dedicated GitHub Actions workflow.
- Consumers should generate client types from the bundled artifact rather than writing parallel request/response TypeScript interfaces.

The current artifact only describes endpoints that exist in this repository. It intentionally does not publish planned compute, network, storage, or billing routes.

## Error, list, and correlation conventions

The shared transport error envelope, pagination bounds, query validation, filtering convention, and request-ID behavior are defined in [PAGINATION.md](PAGINATION.md). Every non-2xx response is normalized to a stable safe error code and message. The response `request_id` is copied only from Loco's sanitized `x-request-id` header.

Domain categories remain framework-independent; HTTP status mapping belongs in the API adapter. Never expose stack traces, raw SQL/provider errors, credentials, or secret-bearing request values.
