"use client";

import { useQuery } from "@tanstack/react-query";

import { fetchConnectivity, getGraphqlUrl } from "../lib/connectivity";

function formatStatus(value: boolean) {
  return value ? "Connected" : "Unavailable";
}

export function ConnectivityPanel() {
  const graphqlUrl = getGraphqlUrl();
  const { data, error, isError, isFetching, isLoading, refetch } = useQuery({
    queryKey: ["connectivity"],
    queryFn: fetchConnectivity,
  });

  if (isLoading) {
    return (
      <section className="status-panel" aria-labelledby="connectivity-heading" aria-busy="true">
        <div>
          <p className="eyebrow">GraphQL connectivity</p>
          <h2 id="connectivity-heading">Checking platform status</h2>
        </div>
        <div className="status-skeleton" aria-label="Loading API and database connectivity">
          <span />
          <span />
          <span />
        </div>
      </section>
    );
  }

  if (isError) {
    return (
      <section className="status-panel status-panel-warning" aria-labelledby="connectivity-heading">
        <div>
          <p className="eyebrow">GraphQL connectivity</p>
          <h2 id="connectivity-heading">Connection needs attention</h2>
        </div>
        <p className="panel-copy">
          The browser could not complete the GraphQL connectivity check. Confirm the API is running,
          then retry this proof from the page.
        </p>
        <dl className="status-list">
          <div>
            <dt>Endpoint</dt>
            <dd>{graphqlUrl}</dd>
          </div>
          <div>
            <dt>Error</dt>
            <dd>{error instanceof Error ? error.message : "Unknown connectivity error"}</dd>
          </div>
        </dl>
        <button className="button" type="button" onClick={() => void refetch()}>
          Retry
        </button>
      </section>
    );
  }

  if (!data) {
    return (
      <section className="status-panel status-panel-warning" aria-labelledby="connectivity-heading">
        <div>
          <p className="eyebrow">GraphQL connectivity</p>
          <h2 id="connectivity-heading">Connection needs attention</h2>
        </div>
        <p className="panel-copy">
          The GraphQL check completed without returning connectivity data. Retry the proof from the
          page.
        </p>
        <button className="button" type="button" onClick={() => void refetch()}>
          Retry
        </button>
      </section>
    );
  }

  const ready = data.status === "ready" && data.databaseReachable && data.migrationsApplied;

  return (
    <section
      className={ready ? "status-panel" : "status-panel status-panel-warning"}
      aria-labelledby="connectivity-heading"
      aria-live="polite"
    >
      <div className="status-heading-row">
        <div>
          <p className="eyebrow">GraphQL connectivity</p>
          <h2 id="connectivity-heading">{ready ? "Platform connected" : "Platform degraded"}</h2>
        </div>
        {isFetching ? <span className="refresh-note">Refreshing</span> : null}
      </div>
      <dl className="status-list">
        <div>
          <dt>GraphQL service</dt>
          <dd>{data.service}</dd>
        </div>
        <div>
          <dt>API status</dt>
          <dd>{data.status}</dd>
        </div>
        <div>
          <dt>PostgreSQL</dt>
          <dd>{formatStatus(data.databaseReachable)}</dd>
        </div>
        <div>
          <dt>SQLx migrations</dt>
          <dd>{formatStatus(data.migrationsApplied)}</dd>
        </div>
        <div>
          <dt>Endpoint</dt>
          <dd>{graphqlUrl}</dd>
        </div>
      </dl>
      <button className="button button-secondary" type="button" onClick={() => void refetch()}>
        Retry
      </button>
    </section>
  );
}
