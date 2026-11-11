const capabilities = [
  {
    name: "Compute",
    description: "Provision and operate virtual machines and workloads.",
    status: "Foundation in progress",
  },
  {
    name: "Network",
    description: "Connect private networks, ingress, DNS, and traffic.",
    status: "Planned",
  },
  {
    name: "Storage",
    description: "Manage object storage, volumes, and snapshots.",
    status: "Planned",
  },
  {
    name: "Security",
    description: "Centralize identity, access policy, and audit history.",
    status: "Foundation in progress",
  },
];

export default function HomePage() {
  const apiBaseUrl =
    process.env.NEXT_PUBLIC_ARSY_API_BASE_URL ?? "Not configured";

  return (
    <main className="min-h-screen overflow-hidden">
      <div className="mx-auto flex min-h-screen w-full max-w-7xl flex-col px-6 py-6 sm:px-10">
        <header className="flex items-center justify-between border-b border-white/10 pb-6">
          <a className="flex items-center gap-3" href="/" aria-label="ARSY home">
            <span className="grid size-10 place-items-center rounded-xl bg-cyan-300 font-black tracking-tight text-slate-950">
              A
            </span>
            <span>
              <span className="block text-lg font-semibold tracking-[0.18em]">
                ARSY
              </span>
              <span className="block text-xs text-slate-400">
                Small Core. Global Infrastructure.
              </span>
            </span>
          </a>
          <span className="rounded-full border border-cyan-300/30 px-3 py-1 text-xs font-medium text-cyan-200">
            Developer preview
          </span>
        </header>

        <section className="grid flex-1 content-center gap-12 py-16 lg:grid-cols-[1.2fr_0.8fr] lg:items-center">
          <div>
            <p className="mb-5 text-sm font-semibold uppercase tracking-[0.24em] text-cyan-300">
              Hybrid cloud control plane
            </p>
            <h1 className="max-w-3xl text-5xl font-semibold leading-[1.05] tracking-tight sm:text-7xl">
              Your infrastructure.
              <span className="block text-cyan-300">One control plane.</span>
            </h1>
            <p className="mt-7 max-w-2xl text-lg leading-8 text-slate-300">
              ARSY is building a unified way to manage infrastructure across
              your own hardware and supported cloud providers. Start with a
              small, secure core. Grow into a global platform.
            </p>
            <div className="mt-9 flex flex-wrap gap-3">
              <a
                className="rounded-lg bg-cyan-300 px-5 py-3 text-sm font-semibold text-slate-950 transition hover:bg-cyan-200"
                href="https://github.com/BRIDGE-ALLAH/arsy"
                target="_blank"
                rel="noreferrer"
              >
                Explore the project
              </a>
              <a
                className="rounded-lg border border-white/15 px-5 py-3 text-sm font-semibold text-white transition hover:bg-white/5"
                href="https://github.com/BRIDGE-ALLAH/arsy/issues/1"
                target="_blank"
                rel="noreferrer"
              >
                View engineering roadmap
              </a>
            </div>
          </div>

          <aside className="rounded-2xl border border-white/10 bg-white/[0.035] p-6 shadow-2xl shadow-cyan-950/20">
            <div className="flex items-center justify-between">
              <p className="text-sm font-medium text-slate-200">
                Platform overview
              </p>
              <span className="size-2 rounded-full bg-amber-300" />
            </div>
            <p className="mt-2 text-xs text-slate-500">
              Early development · Not for production workloads
            </p>
            <div className="mt-7 grid grid-cols-2 gap-3">
              <div className="rounded-xl bg-slate-900/80 p-4">
                <p className="text-xs text-slate-400">Control plane</p>
                <p className="mt-2 text-lg font-semibold">In progress</p>
              </div>
              <div className="rounded-xl bg-slate-900/80 p-4">
                <p className="text-xs text-slate-400">Infrastructure</p>
                <p className="mt-2 text-lg font-semibold">Connecting soon</p>
              </div>
            </div>
            <div className="mt-6 space-y-4">
              {capabilities.map((capability) => (
                <div
                  className="flex items-start justify-between gap-4 border-t border-white/10 pt-4"
                  key={capability.name}
                >
                  <div>
                    <p className="text-sm font-semibold">{capability.name}</p>
                    <p className="mt-1 text-xs leading-5 text-slate-400">
                      {capability.description}
                    </p>
                  </div>
                  <span className="shrink-0 text-right text-[11px] text-slate-500">
                    {capability.status}
                  </span>
                </div>
              ))}
            </div>
          </aside>
        </section>

        <footer className="flex flex-col gap-2 border-t border-white/10 py-5 text-xs text-slate-500 sm:flex-row sm:items-center sm:justify-between">
          <span>© {new Date().getFullYear()} ARSY</span>
          <span className="truncate">API: {apiBaseUrl}</span>
          <span>Built for hybrid infrastructure</span>
        </footer>
      </div>
    </main>
  );
}
