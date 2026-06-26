import { ConnectivityPanel } from "../../components/connectivity-panel";

export default function MarketMapPage() {
  return (
    <div className="market-map-page">
      <section className="page-intro" aria-labelledby="market-map-heading">
        <p className="eyebrow">Analyst workspace</p>
        <h1 id="market-map-heading">Market map</h1>
        <p>
          Foundation route for Tokyo transaction exploration. The map surface is intentionally empty
          until official-source transaction geography is ingested through the pipeline.
        </p>
      </section>

      <div className="workspace-grid">
        <section className="map-panel" aria-labelledby="empty-map-heading">
          <div className="map-toolbar" aria-label="Map layer controls">
            <span>Tokyo transaction geography</span>
            <span>Layer unavailable</span>
          </div>
          <div className="empty-map-state">
            <h2 id="empty-map-heading">No transaction geography loaded</h2>
            <p>
              Slice 4 proves the browser, GraphQL API, migrations, and PostGIS are connected. Public
              transaction records and map layers arrive after ingestion and spatial queries are added.
            </p>
          </div>
        </section>

        <aside className="side-panel" aria-label="Platform state">
          <ConnectivityPanel />
        </aside>
      </div>
    </div>
  );
}
