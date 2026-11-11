# ARSY Frontend

The frontend is a pnpm workspace. Customer-facing applications live under `frontend/apps/*`.

## Console

The Console is a Next.js App Router application using TypeScript and Tailwind CSS.

From the repository root:

```bash
corepack enable
pnpm install
cp frontend/apps/console/.env.example frontend/apps/console/.env.local
pnpm dev:console
```

Open http://localhost:3000.

Useful checks:

```bash
pnpm lint:console
pnpm typecheck:console
pnpm build:console
```

Set `NEXT_PUBLIC_ARSY_API_BASE_URL` to the versioned Control Plane API base URL when connecting API-backed features. Do not put secrets in `NEXT_PUBLIC_*` variables; they are exposed to browser code.

The current Console is an early scaffold and contains no production authentication, tenant data, or infrastructure operations. Do not use it to manage production workloads.
