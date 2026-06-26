export default function MarketMapLoading() {
  return (
    <div className="market-map-page" aria-busy="true">
      <section className="page-intro">
        <p className="eyebrow">Analyst workspace</p>
        <h1>Market map</h1>
        <p>Loading the analyst workspace.</p>
      </section>
      <div className="workspace-grid">
        <section className="map-panel map-panel-loading" aria-label="Loading map workspace" />
        <aside className="side-panel" aria-label="Loading platform state">
          <section className="status-panel">
            <p className="eyebrow">GraphQL connectivity</p>
            <h2>Checking platform status</h2>
            <div className="status-skeleton" aria-label="Loading API and database connectivity">
              <span />
              <span />
              <span />
            </div>
          </section>
        </aside>
      </div>
    </div>
  );
}
