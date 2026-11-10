# Region and compute catalogue

## Data model

- `regions` has a unique code, display name, location label, and explicit `planned`, `enabled`, or `disabled` status.
- `compute_offerings` belongs to a region and includes adapter capability key, vCPU, memory (MiB), storage (GiB), image identifier, and `unverified`, `enabled`, or `disabled` status.
- PostgreSQL enforces foreign keys, uniqueness, valid statuses, positive capacity values, and lookup indexes.

## Read APIs

- `GET /api/v1/catalog/regions`
- `GET /api/v1/catalog/compute-offerings`
- `GET /api/v1/catalog/compute-offerings?region_id=<canonical UUID>`

Both APIs are read-only. Entries include explicit status and a `selectable` flag. A compute offering is selectable only if both its region and the offering itself are enabled. Unknown status values fail closed. Invalid region identifiers return HTTP 400.

The catalogue is global configuration, not a tenant resource. It contains no credentials and does not prove that ARSY owns infrastructure in a named location.

## Repeatable example seed

After migrations have been applied to a local PostgreSQL database:

```bash
./scripts/seed-catalog.sh
```

The seed uses stable identifiers and `ON CONFLICT` clauses, and may be run repeatedly. The only fixture is a planned placeholder region and an unverified compute offering; neither is selectable. Do not enable them without verified infrastructure and a separate operational review.
