const capabilities = [
  {
    title: "Compute",
    description: "Virtual machines and dedicated capacity across connected infrastructure.",
  },
  {
    title: "Network",
    description: "Private networks, ingress, DNS, and traffic controls.",
  },
  {
    title: "Storage",
    description: "Object storage, block volumes, and managed data services.",
  },
];

export default function HomePage() {
  return (
    <main className="page-shell">
      <header className="topbar">
        <a className="wordmark" href="/" aria-label="ARSY home">
          <span className="wordmark-mark" aria-hidden="true">A</span>
          <span>ARSY</span>
        </a>
        <span className="environment-pill">
          <span className="status-dot" aria-hidden="true" />
          Control Plane scaffold
        </span>
      </header>

      <section className="hero" aria-labelledby="page-title">
        <p className="eyebrow">SMALL CORE. GLOBAL INFRASTRUCTURE.</p>
        <h1 id="page-title">Your infrastructure,<br />one control plane.</h1>
        <p className="hero-copy">
          ARSY is building a hybrid cloud platform that brings compute, network,
          and data services together across your own infrastructure and connected providers.
        </p>
        <div className="hero-actions">
          <a className="primary-link" href="https://github.com/BRIDGE-ALLAH/arsy">
            Explore the project <span aria-hidden="true">↗</span>
          </a>
          <span className="build-note">Early development · Not production-ready</span>
        </div>
      </section>

      <section className="capabilities" aria-labelledby="capabilities-title">
        <div className="section-heading">
          <p className="eyebrow">PLATFORM DIRECTION</p>
          <h2 id="capabilities-title">One foundation. Many capabilities.</h2>
        </div>
        <div className="capability-grid">
          {capabilities.map((capability, index) => (
            <article className="capability-card" key={capability.title}>
              <span className="card-index">0{index + 1}</span>
              <h3>{capability.title}</h3>
              <p>{capability.description}</p>
              <span className="planned-label">Planned capability</span>
            </article>
          ))}
        </div>
      </section>

      <footer className="footer">
        <span>ARSY · Hybrid cloud, built deliberately.</span>
        <span>Console foundation</span>
      </footer>
    </main>
  );
}
