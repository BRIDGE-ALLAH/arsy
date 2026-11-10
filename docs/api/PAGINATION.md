# API list and error conventions

## Pagination, filtering, and sorting

List endpoints use these query parameters:

| Parameter | Default | Rules |
|---|---:|---|
| `page` | 1 | One-based positive integer; overflow is rejected |
| `page_size` | 25 | Integer from 1 through 100 |
| `sort` | endpoint-defined | Must be an exact field in that endpoint's allowlist |
| `order` | `desc` | Only `asc` or `desc` |

The response is a stable page envelope: `items`, `page`, `page_size`, `total_pages`, and `total_items`. Empty collections have an empty `items` array and zero total pages. Sorting must include a deterministic unique tie-breaker in each query so records do not jump between adjacent pages.

Filters use `filter[<field>]=<value>`. Each endpoint declares its own filter allowlist, type parser, and authorization-aware query predicates. Unknown fields, unsupported sort keys, malformed types, page zero, page sizes outside 1–100, and arithmetic overflow are rejected. Never interpolate an untrusted sort key or filter name into SQL.

## Error envelope

Every non-2xx ARSY API response is normalized to:

```json
{
  "error": {
    "code": "invalid_request",
    "message": "The request is invalid.",
    "request_id": "9a0b6bb0-93f9-4f4d-a9d0-8c3b0a9d3012"
  }
}
```

Codes are stable machine-readable values: `invalid_request`, `unauthorized`, `forbidden`, `not_found`, `conflict`, `rate_limited`, `unavailable`, and `internal_error`. Messages are fixed safe text. Internal exception text, SQL, provider response bodies, credentials, and stack traces are discarded. Request identifiers are diagnostic correlation only, never authorization credentials.

## Correlation IDs and logs

Loco's built-in `request_id` middleware is intentionally left at its framework default (enabled when the key is omitted). It sanitizes an incoming `x-request-id` or creates a UUID v4, returns it in the `x-request-id` response header, and exposes the ID to structured request logging. The ARSY error normalizer copies only that sanitized identifier into the error envelope. Keep the ID in subsequent operation and audit records when those workflows are implemented; do not accept it as an actor/tenant identity.

The API contract workflow validates the OpenAPI response schema alongside this document.
