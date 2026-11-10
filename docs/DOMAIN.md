# ARSY Domain Primitives

This document defines the stable identifier and error conventions shared by backend domain modules.

## Typed identifiers

The domain uses distinct Rust newtypes for:

- `TenantId`
- `ProjectId`
- `ProviderId`
- `RegionId`
- `ResourceId`
- `OperationId`

Each is backed by a UUID version 4 value. JSON and textual boundaries use the canonical lowercase, hyphenated UUID string (36 characters), for example:

```text
8f14e45f-ea9a-4d59-8f14-e45fea9a4d59
```

Parsing rejects malformed, uppercase, or non-canonical representations. Convert to/from a database UUID only at persistence boundaries; do not pass raw strings between domain APIs.

## Domain error categories

`DomainError` is independent of Loco and HTTP. It distinguishes:

| Category | Meaning |
|---|---|
| `validation` | Invalid domain input or identifier |
| `not_found` | A requested domain entity does not exist |
| `forbidden` | The actor is not permitted to perform the domain action |
| `conflict` | A uniqueness, idempotency, or state-transition constraint is violated |
| `retryable` | A known temporary condition may be retried according to policy |
| `provider` | A provider adapter returned a classified failure; retryability is explicit |

Serialized errors contain only stable category/code values. They never contain raw provider messages, credentials, SQL errors, or stack traces. Mapping to HTTP status codes belongs to the transport/API adapter, not this module.

## Module boundary

Implementation lives in `backend/apps/arsy-control-plane/src/domain`. It depends only on standard Rust types plus serde/UUID primitives; it must not import Loco controllers, Axum, SeaORM, or provider SDKs.
