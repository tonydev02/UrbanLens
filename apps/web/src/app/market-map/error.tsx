"use client";

export default function MarketMapError({
  error,
  reset,
}: Readonly<{
  error: Error & { digest?: string };
  reset: () => void;
}>) {
  return (
    <section className="route-state" aria-labelledby="route-error-heading">
      <p className="eyebrow">Analyst workspace</p>
      <h1 id="route-error-heading">Market map could not load</h1>
      <p>{error.message || "The route failed before the connectivity panel could render."}</p>
      <button className="button" type="button" onClick={reset}>
        Retry
      </button>
    </section>
  );
}
