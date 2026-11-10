export default function Home() {
  return <main className="mx-auto flex min-h-screen max-w-6xl flex-col justify-center px-6 py-24 text-center">
    <div className="mx-auto mb-8 grid size-14 place-items-center rounded-2xl border border-blue-200/30 bg-blue-300/10 text-xl font-bold text-blue-100">A</div>
    <p className="text-xs font-semibold uppercase tracking-[0.32em] text-blue-200">Independent hybrid cloud</p>
    <h1 className="mx-auto mt-5 max-w-4xl text-5xl font-semibold tracking-tight text-white md:text-7xl">Small Core.<br /><span className="text-blue-200">Global Infrastructure.</span></h1>
    <p className="mx-auto mt-7 max-w-2xl text-lg leading-8 text-slate-400">One control plane for infrastructure across ARSY-operated resources and supported third-party providers.</p>
    <div className="mt-10 flex justify-center"><span className="rounded-full border border-amber-200/20 bg-amber-200/10 px-4 py-2 text-sm text-amber-100">Early development · Services are not yet available</span></div>
    <footer className="mt-24 text-xs text-slate-600">ARSY is under active development.</footer>
  </main>;
}
