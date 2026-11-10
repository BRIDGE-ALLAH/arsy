# ARSY Frontend

The frontend is a pnpm workspace. The first application is the Next.js App Router console at `apps/console`.

## Console

From the repository root:

```bash
pnpm install
pnpm dev:console
```

The console uses Next.js App Router, React, and strict TypeScript. It is a product foundation only: authentication, navigation shell, API integration, and cloud resource pages are separate implementation tasks.

Copy `frontend/apps/console/.env.example` to `frontend/apps/console/.env.local` to configure the public Control Plane API base URL. Only non-secret public configuration belongs in `NEXT_PUBLIC_*` variables.

## Quality checks

```bash
pnpm lint:console
pnpm typecheck:console
pnpm build:console
```

## Security rules

- Backend authorization is authoritative; hiding a UI action is not security.
- Never store provider credentials in browser storage.
- Do not put server-side secrets in `NEXT_PUBLIC_*` variables.
- Prefer generated OpenAPI types once the API contract stabilizes.
- Keep server components for server-rendered data and client components for interactive controls.
- Add accessible loading, empty, error, and permission-denied states.
