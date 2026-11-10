# ARSY Control Plane API

## Versioning policy

New customer-facing API endpoints are added under `/api/v1`. A breaking change requires a new major path version (for example, `/api/v2`) and a documented migration/deprecation window. Additive optional response fields and new endpoints may be introduced within a version when compatibility is preserved.

The original generated authentication routes under `/api/auth` remain temporarily available as a compatibility alias. New clients should use `/api/v1/auth`. Do not remove the alias without a separately reviewed deprecation notice and migration plan.

## OpenAPI

The checked-in OpenAPI 3.1 document is:

- `backend/apps/arsy-control-plane/openapi.json`

It is served by the Control Plane at:

- `GET /api/v1/openapi.json`

The document currently describes the authentication endpoints and bearer-token requirement. It is the initial contract, not a complete specification for future compute, network, storage, or billing APIs. Each new endpoint must update the document and include request/response schemas, error responses, and authorization requirements.

## Authentication

The preferred versioned endpoints are `/api/v1/auth/*`. Requests that require a session use `Authorization: Bearer <JWT>`. Browser clients must not put provider credentials or long-lived privileged tokens in local storage.

The generated register/verify/login/recovery flow is an early account-authentication foundation. It is not a complete organization membership or authorization model. Do not expose provider resources until organization/project RBAC and tenant isolation are enforced by the backend.
