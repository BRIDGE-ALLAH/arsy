const products = [
  { name: "Compute", detail: "Virtual machines and nodes" },
  { name: "Network", detail: "Private networks, IPs, and ingress" },
  { name: "Storage", detail: "Object and block storage" },
  { name: "Databases", detail: "Managed data services" },
];
export default function ConsoleHome() {
  return <main className="mx-auto flex min-h-screen w-full max-w-7xl flex-col px-6 py-8 md:px-10">
    <header className="flex flex-wrap items-center justify-between gap-4 border-b border-[var(--border)] pb-6">
      <div className="flex items-center gap-3"><div className="grid size-10 place-items-center rounded-xl border border-blue-300/30 bg-blue-400/10 text-sm font-bold text-blue-200">A</div><div><p className="text-sm font-semibold tracking-[0.22em] text-white">ARSY</p><p className="text-xs text-[var(--muted)]">Small Core. Global Infrastructure.</p></div></div>
      <span className="rounded-full border border-amber-300/20 bg-amber-300/10 px-3 py-1 text-xs text-amber-100">Development preview</span>
    </header>
    <section className="py-12 md:py-16"><p className="mb-3 text-xs font-semibold uppercase tracking-[0.24em] text-blue-200">Control plane</p><h1 className="max-w-3xl text-4xl font-semibold tracking-tight text-white md:text-6xl">Your infrastructure, <span className="text-blue-200">one control plane.</span></h1><p className="mt-5 max-w-2xl text-base leading-7 text-[var(--muted)] md:text-lg">ARSY is being built to manage compute and infrastructure across owned hardware and supported cloud providers.</p></section>
    <section className="grid gap-4 sm:grid-cols-2 xl:grid-cols-4" aria-label="Infrastructure product areas">
      {products.map((item) => <article key={item.name} className="rounded-2xl border border-[var(--border)] bg-[var(--panel)]/80 p-5"><div className="mb-8 flex items-start justify-between gap-3"><h2 className="text-lg font-semibold text-white">{item.name}</h2><span className="rounded-full border border-slate-500/30 px-2 py-1 text-[10px] uppercase tracking-wide text-slate-300">Planned</span></div><p className="text-sm leading-6 text-[var(--muted)]">{item.detail}</p></article>)}
    </section>
    <section className="mt-8 rounded-2xl border border-[var(--border)] bg-slate-950/50 p-6"><h2 className="text-sm font-semibold text-white">Platform status</h2><p className="mt-2 text-sm leading-6 text-[var(--muted)]">This is an early scaffold. No cloud resources can be provisioned yet. Authentication, organization isolation, provider connections, and compute lifecycle follow in later phases.</p></section>
    <footer className="mt-auto pt-12 text-xs text-slate-500">ARSY · Engineering preview · Not a production cloud service</footer>
  </main>;
}
