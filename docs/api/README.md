# ARSY API Contract

## Versioning policy

All ARSY-owned REST API routes live under `/api/v1`. Compatible additive changes may be made within a version. A breaking change requires a new major path prefix (for example, `/api/v2`) with an explicit migration/deprecation plan. Framework-owned operational probes such as Loco's `/_ping`, `/_health`, and `/_readiness` stay outside the application API version prefix.

The first application routes are mounted at:

- `/api/v1/auth/*`: the existing Loco authentication handlers.
- `/api/v1/health/ready`: readiness for the required PostgreSQL dependency.
- `/_ping`: Loco's built-in process liveness response (`{"ok":true}`).

## OpenAPI source and artifact

- Canonical source: `docs/api/source/openapi.yaml`.
- Bundled/reference artifact: `docs/api/openapi.yaml`.
- Generator: `scripts/generate-openapi.sh` copies the source deterministically. When local references are introduced, this script is the single place to replace the copy step with a pinned bundler invocation.
- Validation: Redocly CLI is pinned to an exact version in the dedicated GitHub Actions workflow.
- Consumers should generate client types from the bundled artifact rather than writing parallel request/response TypeScript interfaces.

The current artifact only describes endpoints that exist in this repository. It intentionally does not publish planned compute, network, storage, or billing routes.

## Error envelope

Transport errors retain the current Loco-safe envelope shape:

```json
{
  "error": "unauthorized",
  "description": "You do not have permission to access this resource"
}
```

Domain categories remain framework-independent; HTTP status mapping belongs in the API adapter. Never expose stack traces, raw SQL/provider errors, credentials, or secret-bearing request values.
