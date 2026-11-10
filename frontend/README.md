# ARSY Frontend

The planned customer console and public website use Next.js App Router and TypeScript.

## Applications

- apps/console: authenticated cloud management console.
- apps/website: public product and documentation entry point.

## Shared packages (add as needed)

- packages/ui: design-system components.
- packages/api-client: generated or typed Control Plane API client.
- packages/types: shared API and UI types.
- packages/config: shared lint and TypeScript configuration.

## Recommended stack

- Next.js App Router + TypeScript
- Tailwind CSS + shadcn/ui
- TanStack Query for remote state
- Zustand only for local client-owned state
- React Hook Form + Zod
- Apache ECharts for metrics
- pnpm

## Rules

- Backend authorization is authoritative; hiding a UI action is not security.
- Never store provider credentials in browser storage.
- Prefer generated OpenAPI types once the API contract stabilizes.
- Keep server components for server-rendered data and client components for interactive controls.
- Add accessible loading, empty, error, and permission-denied states.

## Status

The Next.js applications are planned but have not yet been scaffolded or built in this repository.